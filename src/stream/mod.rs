extern crate coreaudio_sys as sys;

use std::marker::PhantomData;
use std::mem::size_of;
use std::os::raw::c_void;
use std::slice;

mod audio_unit;

use self::audio_unit::{AudioUnit, Element};

#[derive(Debug)]
pub enum Error {
    AudioUnit(audio_unit::Error),
}

// To convert a audio_unit::Error to a Error.
impl From<audio_unit::Error> for Error {
    fn from(e: audio_unit::Error) -> Self {
        Error::AudioUnit(e)
    }
}

// TODO: Use native type to infer format directly.
pub enum Format {
    S16LE, // PCM signed 16-bit little-endian.
    F32LE, // PCM 32-bit floating-point little-endian.
}

impl Format {
    fn byte_size(&self) -> usize {
        match self {
            Format::S16LE => size_of::<i16>(),
            Format::F32LE => size_of::<f32>(),
        }
    }

    fn to_format_flags(&self) -> sys::AudioFormatFlags {
        let flags = match self {
            Format::S16LE => sys::kAudioFormatFlagIsSignedInteger,
            Format::F32LE => sys::kAudioFormatFlagIsFloat,
        };
        flags |
        sys::kLinearPCMFormatFlagIsPacked |
        sys::kLinearPCMFormatFlagIsNonInterleaved
    }
}

struct Parameters {
    channels: u32,
    format: Format,
    rate: f64,
}
impl Parameters {
    fn new(channels: u32, format: Format, rate: f64) -> Self {
        Parameters {
            channels,
            format,
            rate,
        }
    }
    fn to_description(&self) -> sys::AudioStreamBasicDescription {
        let byte_size = self.format.byte_size() as u32;
        let bits_per_channel = byte_size * 8;
        let frames_per_packet = 1;
        // The channels in the buffer is set to non-interleaved by
        // AudioFormatFlags here, hence the `bytes_per_frame` is same as
        // `bytes_per_frame` and `AudioBufferList.mNumberBuffers` received from
        // callback `audio_unit_render_callback` is same as the number of
        // channels we have.
        let bytes_per_frame = byte_size;
        let bytes_per_packet = bytes_per_frame * frames_per_packet;
        sys::AudioStreamBasicDescription {
            mSampleRate: self.rate,
            mFormatID: sys::kAudioFormatLinearPCM,
            mFormatFlags: self.format.to_format_flags(),
            mBytesPerPacket: bytes_per_packet,
            mFramesPerPacket: frames_per_packet,
            mBytesPerFrame: bytes_per_frame,
            mChannelsPerFrame: self.channels,
            mBitsPerChannel: bits_per_channel,
            mReserved: 0,
        }
    }
}

// A wrapper around the pointer to the `AudioBufferList::mBuffers` array.
// Using `PhantomData` to carry the target type when passing this struct
// from functions to functions.
struct AudioData<T> {
    buffers: &'static mut [sys::AudioBuffer], // The list of audio buffers.
    frames: usize, // The number of frames in each channel.
    data_type: PhantomData<T>
}

pub type CallbackArgs<'a, T> = &'a mut [&'a mut [T]];
type Callback<T> = fn(CallbackArgs<T>);

// The Stream struct will be converted to a pointer and the pointer will be
// set as a `custom data` pointer to the underlying `AudioUnit` callback
// function. (see `inputProcRefCon` in `set_callback`). Since underlying
// function is implemented in C, using `#[repr(C)]` to prevent the struct
// layout of `Stream` from being mangled by Rust compiler.
#[repr(C)]
pub struct Stream<T> {
    callback: Callback<T>,
    parameters: Parameters,
    unit: AudioUnit,
}

// Learn AUHAL concepts of `scope` and `bus (element)` from below link:
// https://developer.apple.com/library/archive/technotes/tn2091/_index.html
// This gives idea about how we set the audio stream here.
impl<T> Stream<T> {
    pub fn new(channels: u32, format: Format, rate: f64, callback: Callback<T>) -> Result<Self, Error> {
        assert_eq!(format.byte_size(), size_of::<T>());
        let parameters = Parameters::new(channels, format, rate);
        let unit = AudioUnit::new()?;
        let stm = Stream { callback, parameters, unit };
        // Don't initialize the stream here!
        // The memory address of `stm` is different from `x`
        // where x is returned `stm` outside by `x = Stream::new(...)`.
        // If we call `stm.init()` or `stm.set_callback()` here, the `self` of
        // `stm.init()` or `set_callback` are `stm` and it will be freed
        // after `stm` is returned from `new`. Hence the `inputProcRefCon`
        // in `set_callback` will be assigned to a dangling pointer and lead
        // a segment fault or bus error when trying to use `in_ref_con`, which
        // is a pointer pointing a freed memory chunk, in
        // `audio_unit_render_callback`.
        Ok(stm)
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.set_stream_format()?;
        self.set_callback()?;
        self.init_unit()?;
        Ok(())
    }

    pub fn start(&self) -> Result<(), Error> {
        self.unit.start()?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), Error> {
        self.unit.stop()?;
        Ok(())
    }

    fn init_unit(&self) -> Result<(), Error> {
        self.unit.initialize()?;
        Ok(())
    }

    fn uninit_unit(&self) -> Result<(), Error> {
        self.unit.uninitialize()?;
        Ok(())
    }

    fn set_stream_format(&self) -> Result<(), Error> {
        let description = self.parameters.to_description();
        self.unit.set_property(
            sys::kAudioUnitProperty_StreamFormat,
            sys::kAudioUnitScope_Input,
            Element::Output,
            &description,
        )?;
        Ok(())
    }

    fn set_callback(&mut self) -> Result<(), Error> {
        let callback_struct = sys::AURenderCallbackStruct {
            inputProc: Some(audio_unit_render_callback::<Self>),
            inputProcRefCon: self as *mut Self as *mut c_void,
        };

        self.unit.set_property(
            sys::kAudioUnitProperty_SetRenderCallback,
            sys::kAudioUnitScope_Input,
            Element::Output,
            &callback_struct,
        )?;
        Ok(())
    }

    fn get_buffer_data (&self, data: AudioData<T>) -> sys::OSStatus {
        let mut channel_buffers = Vec::new();
        assert_eq!(data.buffers.len() as u32, self.parameters.channels);
        for buffer in data.buffers {
            assert_eq!(buffer.mNumberChannels, 1);
            assert_eq!(
                (data.frames * size_of::<T>()) as u32,
                buffer.mDataByteSize
            );
            let ptr = buffer.mData as *mut T;
            let len = data.frames;
            let channel_buffer = unsafe { slice::from_raw_parts_mut(ptr, len) };
            channel_buffers.push(channel_buffer);
        }
        (self.callback)(&mut channel_buffers);
        sys::noErr as sys::OSStatus
    }
}

impl<T> Drop for Stream<T> {
    fn drop(&mut self) {
        assert!(self.stop().is_ok());
        assert!(self.uninit_unit().is_ok());
    }
}

// This trait will be used when
// https://developer.apple.com/documentation/audiotoolbox/aurendercallback?language=objc
trait RenderCallback {
    fn render(
        &self,
        io_action_flags: *mut sys::AudioUnitRenderActionFlags,
        in_time_stamp: *const sys::AudioTimeStamp,
        in_bus_number: sys::UInt32,
        in_number_of_frames: sys::UInt32,
        io_data: *mut sys::AudioBufferList
    ) -> sys::OSStatus;
}

impl<T> RenderCallback for Stream<T> {
    fn render(
        &self,
        io_action_flags: *mut sys::AudioUnitRenderActionFlags,
        in_time_stamp: *const sys::AudioTimeStamp,
        in_bus_number: sys::UInt32,
        in_number_of_frames: sys::UInt32,
        io_data: *mut sys::AudioBufferList
    ) -> sys::OSStatus {
        let buffers = unsafe {
            let ptr = (*io_data).mBuffers.as_ptr() as *mut sys::AudioBuffer;
            let len = (*io_data).mNumberBuffers as usize; // interleaved channels.
            slice::from_raw_parts_mut(ptr, len)
        };
        let data = AudioData {
            buffers: buffers,
            frames: in_number_of_frames as usize,
            data_type: PhantomData,
        };
        self.get_buffer_data(data)
    }
}

// The static callback function that will be registered by
// `AURenderCallbackStruct` and called by underlying `AudioUnit` framework
// directly.
// see:
// https://developer.apple.com/documentation/audiotoolbox/aurendercallbackstruct?language=objc
// https://developer.apple.com/documentation/audiotoolbox/aurendercallback?language=objc
//
// The type `R: RenderCallback` is used to checked the `in_ref_con` is an
// object that implements `render` function.
extern "C" fn audio_unit_render_callback<R>(
    in_ref_con: *mut c_void,
    io_action_flags: *mut sys::AudioUnitRenderActionFlags,
    in_time_stamp: *const sys::AudioTimeStamp,
    in_bus_number: sys::UInt32,
    in_number_of_frames: sys::UInt32,
    io_data: *mut sys::AudioBufferList,
) -> sys::OSStatus where R: RenderCallback {
    let render_callback_object = in_ref_con as *mut R;
    unsafe {
        (*render_callback_object).render(
            io_action_flags,
            in_time_stamp,
            in_bus_number,
            in_number_of_frames,
            io_data,
        )
    }
}