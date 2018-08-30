extern crate coreaudio_sys as sys;

use std::marker::PhantomData;
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
    fn from(e: audio_unit::Error) -> Error {
        Error::AudioUnit(e)
    }
}

pub enum Format {
    S16LE, // PCM signed 16-bit little-endian.
    F32LE, // PCM 32-bit floating-point little-endian.
}

impl Format {
    pub fn to_bits_per_channels(&self) -> u32 {
        match self {
            Format::S16LE => 16,
            Format::F32LE => 32,
        }
    }

    pub fn to_format_flags(&self) -> sys::AudioFormatFlags {
        let flags = match self {
            Format::S16LE => sys::kAudioFormatFlagIsSignedInteger,
            Format::F32LE => sys::kAudioFormatFlagIsFloat,
        };
        flags | sys::kLinearPCMFormatFlagIsPacked
    }
}

pub struct Parameters {
    channels: u32,
    format: Format,
    rate: f64,
}
impl Parameters {
    fn new(channels: u32, format: Format, rate: f64) -> Parameters {
        Parameters {
            channels,
            format,
            rate,
        }
    }
    fn to_description(&self) -> sys::AudioStreamBasicDescription {
        let bits_per_channel = self.format.to_bits_per_channels();
        let frames_per_packet = 1;
        let bytes_per_frame = (bits_per_channel / 8) * self.channels;
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

pub type Callback = FnMut(
    *mut sys::AudioUnitRenderActionFlags,
    *const sys::AudioTimeStamp,
    sys::UInt32,
    sys::UInt32,
    *mut sys::AudioBufferList
) -> sys::OSStatus;

pub struct CallbackWrapper {
    callback: Box<Callback>,
}

pub struct ChannelsMut<'a, S: 'a> {
    buffers: slice::IterMut<'a, sys::AudioBuffer>,
    frames: usize,
    sample_format: PhantomData<S>,
}
impl<'a, S> Iterator for ChannelsMut<'a, S> {
    type Item = &'a mut [S];
    #[allow(non_snake_case)]
    fn next(&mut self) -> Option<Self::Item> {
        self.buffers.next().map(|&mut sys::AudioBuffer { mNumberChannels, mData, .. }| {
            let len = mNumberChannels as usize * self.frames;
            let ptr = mData as *mut S;
            unsafe { slice::from_raw_parts_mut(ptr, len) }
        })
    }
}

// A wrapper around the pointer to the `AudioBufferList::mBuffers` array.
pub struct Buffer<T> {
    // The list of audio buffers.
    buffers: &'static mut [sys::AudioBuffer],
    // The number of frames in each channel.
    frames: usize,
    data_format: PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn channels_mut(&mut self) -> ChannelsMut<T> {
        ChannelsMut {
            buffers: self.buffers.iter_mut(),
            frames: self.frames,
            sample_format: PhantomData,
        }
    }
}

pub trait Data {
    fn from_input_proc_args(num_frames: u32, io_data: *mut sys::AudioBufferList) -> Self;
}

impl<T> Data for Buffer<T> {
    fn from_input_proc_args(frames: u32, io_data: *mut sys::AudioBufferList) -> Self {
        let buffers = unsafe {
            let ptr = (*io_data).mBuffers.as_ptr() as *mut sys::AudioBuffer;
            let len = (*io_data).mNumberBuffers as usize;
            slice::from_raw_parts_mut(ptr, len)
        };
        Buffer {
            buffers: buffers,
            frames: frames as usize,
            data_format: PhantomData,
        }
    }
}

pub struct CallbackArgs<D> {
    pub data: D, // The expected type for data in the buffer.
    pub frames: usize, // The number of frames in the buffer.
}

pub struct Stream {
    unit: AudioUnit,
}

impl Stream {
    pub fn new<F, D>(channels: u32, format: Format, rate: f64, callback: F) -> Result<Self, Error>
    where
        F: FnMut(CallbackArgs<D>) + 'static,
        D: Data,
    {
        let params = Parameters::new(channels, format, rate);
        let unit = AudioUnit::new()?;
        let mut stm = Stream { unit };
        stm.set_stream_format(&params)?;
        stm.set_callback(callback)?;
        stm.init()?;
        Ok(stm)
    }

    pub fn start(&self) -> Result<(), Error> {
        self.unit.start()?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), Error> {
        self.unit.stop()?;
        Ok(())
    }

    fn set_callback<F, D>(&mut self, mut f: F) -> Result<(), Error>
    where
        F: FnMut(CallbackArgs<D>) + 'static,
        D: Data,
    {
        let callback = move |io_action_flags: *mut sys::AudioUnitRenderActionFlags,
                                  in_time_stamp: *const sys::AudioTimeStamp,
                                  in_bus_number: sys::UInt32,
                                  in_number_frames: sys::UInt32,
                                  io_data: *mut sys::AudioBufferList| -> sys::OSStatus
        {
            let data = D::from_input_proc_args(in_number_frames, io_data);
            let args = CallbackArgs {
                data: data,
                frames: in_number_frames as usize,
            };

            f(args);
            sys::noErr as sys::OSStatus
        };

        let callback_wrapper = Box::new(CallbackWrapper {
            callback: Box::new(callback),
        });
        let callback_wrapper_ptr = Box::into_raw(callback_wrapper) as *mut c_void;

        let callback_struct = sys::AURenderCallbackStruct {
            inputProc: Some(input_proc),
            inputProcRefCon: callback_wrapper_ptr,
        };

        self.unit.set_property(
            sys::kAudioUnitProperty_SetRenderCallback,
            sys::kAudioUnitScope_Input,
            Element::Output,
            &callback_struct,
        )?;
        Ok(())
    }

    fn set_stream_format(&self, params: &Parameters) -> Result<(), Error> {
        self.unit.set_property(
            sys::kAudioUnitProperty_StreamFormat,
            sys::kAudioUnitScope_Global,
            Element::Output,
            &params.to_description(),
        )?;
        Ok(())
    }

    fn init(&self) -> Result<(), Error> {
        self.unit.initialize()?;
        Ok(())
    }

    fn uninit(&self) -> Result<(), Error> {
        self.unit.uninitialize()?;
        Ok(())
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        self.stop();
        self.uninit();
    }
}

extern "C" fn input_proc(
    in_ref_con: *mut c_void,
    io_action_flags: *mut sys::AudioUnitRenderActionFlags,
    in_time_stamp: *const sys::AudioTimeStamp,
    in_bus_number: sys::UInt32,
    in_number_of_frames: sys::UInt32,
    io_data: *mut sys::AudioBufferList
) -> sys::OSStatus {
    let wrapper = in_ref_con as *mut CallbackWrapper;
    unsafe {
        (*(*wrapper).callback)(io_action_flags,
                               in_time_stamp,
                               in_bus_number,
                               in_number_of_frames,
                               io_data)
    }
}