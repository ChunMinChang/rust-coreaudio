extern crate coreaudio_sys;

mod audio_object_utils;
mod property_address;
mod string_wrapper;

use self::coreaudio_sys::{
    AudioBuffer,
    AudioBufferList,
    AudioObjectPropertyAddress,
    AudioObjectID,
    AudioValueRange,
    kAudioObjectSystemObject,   // AudioObjectID
    kAudioObjectUnknown,        // AudioObjectID
    AudioStreamID,              // AudioObjectID
    AudioValueTranslation,
};
use self::property_address::{
    DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
    DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
    DEVICE_MANUFACTURER_PROPERTY_ADDRESS,
    DEVICE_NAME_PROPERTY_ADDRESS,
    DEVICE_UID_PROPERTY_ADDRESS,
    DEVICES_PROPERTY_ADDRESS,
    INPUT_DEVICE_AVAILABLE_SAMPLE_RATE_PROPERTY_ADDRESS,
    INPUT_DEVICE_BUFFER_FRAME_SIZE_RANGE_PROPERTY_ADDRESS,
    INPUT_DEVICE_LATENCY_PROPERTY_ADDRESS,
    INPUT_DEVICE_SAMPLE_RATE_PROPERTY_ADDRESS,
    INPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS,
    INPUT_DEVICE_SOURCE_PROPERTY_ADDRESS,
    INPUT_DEVICE_STREAMS_PROPERTY_ADDRESS,
    INPUT_DEVICE_STREAM_CONFIGURATION_PROPERTY_ADDRESS,
    INPUT_STREAM_LATENCY_PROPERTY_ADDRESS,
    OUTPUT_DEVICE_AVAILABLE_SAMPLE_RATE_PROPERTY_ADDRESS,
    OUTPUT_DEVICE_BUFFER_FRAME_SIZE_RANGE_PROPERTY_ADDRESS,
    OUTPUT_DEVICE_LATENCY_PROPERTY_ADDRESS,
    OUTPUT_DEVICE_SAMPLE_RATE_PROPERTY_ADDRESS,
    OUTPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS,
    OUTPUT_DEVICE_SOURCE_PROPERTY_ADDRESS,
    OUTPUT_DEVICE_STREAMS_PROPERTY_ADDRESS,
    OUTPUT_DEVICE_STREAM_CONFIGURATION_PROPERTY_ADDRESS,
    OUTPUT_STREAM_LATENCY_PROPERTY_ADDRESS,
};
use self::string_wrapper::StringRef;

use std::f64; // For f64::{MAX, MIN}
use std::fmt; // For fmt::{Debug, Formatter, Result}
use std::mem; // For mem::size_of()
use std::os::raw::c_void;
use std::slice;

// TODO: Maybe we should move this enum out since other module may also
//       need the scope.
// Using PartialEq for comparison.
#[derive(PartialEq)]
pub enum Scope {
    Input,
    Output,
}

// Using PartialEq for comparison.
#[derive(PartialEq)]
pub enum Error {
    ConversionFailed(string_wrapper::Error),
    InvalidParameters(audio_object_utils::Error),
    NoDeviceFound,
    SetSameDevice,
    WrongScope,
}

// To convert an audio_object_utils::Error to a Error.
impl From<audio_object_utils::Error> for Error {
    fn from(e: audio_object_utils::Error) -> Error {
        Error::InvalidParameters(e)
    }
}

// To convert an string_wrapper::Error to a Error.
impl From<string_wrapper::Error> for Error {
    fn from(e: string_wrapper::Error) -> Error {
        Error::ConversionFailed(e)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Error::ConversionFailed(e) => format!("Fail to convert string: {:?}", e),
            Error::InvalidParameters(e) => format!("Invalid parameters: {:?}", e),
            Error::NoDeviceFound => "No valid device found by given information.".to_string(),
            Error::SetSameDevice => "Try setting the device with the same one".to_string(),
            Error::WrongScope => "The given scope is wrong.".to_string(),
        };
        write!(f, "{}", printable)
    }
}

// TODO: Use macro to the struct needs the following traits.
// Commom traits for the wrappers struct of Audio*-type
// ============================================================================
pub trait GetObjectId {
    fn get_id(&self) -> AudioObjectID;
}

trait GetPropertyData {
    fn get_property_data<T: Default>(
        &self,
        address: &AudioObjectPropertyAddress,
    ) -> Result<T, Error>
        where Self: GetObjectId
    {
        audio_object_utils::get_property_data::<T>(
            self.get_id(),
            address
        ).map_err(|e| e.into())
    }
}

trait GetPropertyDataWithPtr {
    fn get_property_data_with_ptr<T>(
        &self,
        address: &AudioObjectPropertyAddress,
        data: &mut T,
    ) -> Result<(), Error>
        where Self: GetObjectId
    {
        audio_object_utils::get_property_data_with_ptr(
            self.get_id(),
            address,
            data
        ).map_err(|e| e.into())
    }
}

trait GetPropertyDataSize {
    fn get_property_data_size(
        &self,
        address: &AudioObjectPropertyAddress,
    ) -> Result<usize, Error>
        where Self: GetObjectId
    {
        audio_object_utils::get_property_data_size(
            self.get_id(),
            address
        ).map_err(|e| e.into())
    }
}

trait GetPropertyArray {
    fn get_property_array<T>(
        &self,
        address: &AudioObjectPropertyAddress,
    ) -> Result<Vec<T>, Error>
        where Self: GetObjectId
    {
        audio_object_utils::get_property_array::<T>(
            self.get_id(),
            address
        ).map_err(|e| e.into())
    }
}

trait GetPropertyVeriableSizedData {
    fn get_property_variable_sized_data<'a, T>(
        &self,
        address: &AudioObjectPropertyAddress,
    ) -> Result<&'a T, Error>
        where Self: GetObjectId
    {
        audio_object_utils::get_property_variable_sized_data::<T>(
            self.get_id(),
            address
        ).map_err(|e| e.into())
    }
}

trait SetPropertyData {
    fn set_property_data<T>(
        &self,
        address: &AudioObjectPropertyAddress,
        data: &T,
    ) -> Result<(), Error>
        where Self: GetObjectId
    {
        audio_object_utils::set_property_data(
            self.get_id(),
            address,
            data
        ).map_err(|e| e.into())
    }
}

// AudioSystemObject
// ============================================================================
pub struct AudioSystemObject(AudioObjectID);

impl AudioSystemObject {
    pub fn new() -> Self {
        AudioSystemObject(kAudioObjectSystemObject)
    }

    pub fn get_default_device(
        &self,
        scope: &Scope
    ) -> Result<AudioObject, Error> {
        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS
        } else {
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS
        };
        let device: AudioObject = self.get_property_data(address)?;
        // We will get an unknow device when there is no available device at
        // this time
        if device.is_valid() {
            Ok(device)
        } else {
            Err(Error::NoDeviceFound)
        }
    }

    // Apple has no API to get input-only or output-only devices. To do that,
    // we need to get all the devices first ans then check if they are input
    // or output one by one.
    pub fn get_devices(
        &self,
        scope: &Scope
    ) -> Result<Vec<AudioObject>, Error> {
        let mut devices: Vec<AudioObject> = self.get_all_devices()?;
        // It's ok to call `unwrap()` here since all the `AudioObjectID` values
        // in `devices` are valid.
        devices.retain(|ref device| device.in_scope(scope).unwrap());
        Ok(devices)
    }

    pub fn get_all_devices(&self) -> Result<Vec<AudioObject>, Error> {
        self.get_property_array::<AudioObject>(
            &DEVICES_PROPERTY_ADDRESS,
        ).map_err(|e| e.into())
    }

    pub fn set_default_device(
        &self,
        device: &AudioObject,
        scope: &Scope
    ) -> Result<(), Error> {
        // Surprisingly it's ok to set
        //   1. a unknown device
        //   2. a non-input/non-output device
        //   3. the current default input/output device
        // as the new default input/output device by apple's API.
        // We need to check the above things by ourselves.
        if !device.in_scope(scope)? {
            return Err(Error::WrongScope);
        }

        let default_device = self.get_default_device(scope)?;
        if device == &default_device {
            return Err(Error::SetSameDevice);
        }

        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS
        } else {
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS
        };
        self.set_property_data(address, device.into()).map_err(|e| e.into())
    }
}

impl GetObjectId for AudioSystemObject {
    fn get_id(&self) -> AudioObjectID {
        self.0
    }
}

impl GetPropertyData for AudioSystemObject {}
impl SetPropertyData for AudioSystemObject {}
impl GetPropertyArray for AudioSystemObject {}

// AudioObject
// ============================================================================
#[derive(Clone, Debug, PartialEq)]
pub struct AudioObject(AudioObjectID);

// TODO: remove or add 'device' for all the function names.
impl AudioObject {
    pub fn new(id: AudioObjectID) -> Self {
        AudioObject(id)
    }

    pub fn is_valid(&self) -> bool {
        self.0 != kAudioObjectUnknown
    }

    pub fn get_channel_count(
        &self,
        scope: &Scope
    ) -> Result<u32, Error> {
        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &INPUT_DEVICE_STREAM_CONFIGURATION_PROPERTY_ADDRESS
        } else {
            &OUTPUT_DEVICE_STREAM_CONFIGURATION_PROPERTY_ADDRESS
        };
        // Calculate number of channels by the AudioBufferList.
        // The mNumberBuffers is the number of interleaved channels in the buffer.
        // The buffer is noninterleaved if mNumberBuffers is 1.
        let list: &AudioBufferList = self.get_property_variable_sized_data(address)?;
        let buffers = unsafe {
            let ptr = list.mBuffers.as_ptr() as *mut AudioBuffer;
            let len = list.mNumberBuffers as usize; // interleaved channels.
            slice::from_raw_parts_mut(ptr, len)
        };
        let mut count = 0;
        for buffer in buffers {
            // mNumberChannels is the number of interleaved channels in the buffer.
            count += buffer.mNumberChannels;
        }
        Ok(count)
    }

    pub fn get_uid(&self) -> Result<String, Error> {
        let uid: StringRef =
            self.get_property_data(&DEVICE_UID_PROPERTY_ADDRESS)?;
        uid.into_string().map_err(Error::ConversionFailed)
    }

    pub fn get_manufacturer(&self) -> Result<String, Error> {
        let manufacturer: StringRef =
            self.get_property_data(&DEVICE_MANUFACTURER_PROPERTY_ADDRESS)?;
        manufacturer.into_string().map_err(Error::ConversionFailed)
    }

    pub fn get_default_rate(
        &self,
        scope: &Scope
    ) -> Result<f64, Error> {
        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &INPUT_DEVICE_SAMPLE_RATE_PROPERTY_ADDRESS
        } else {
            &OUTPUT_DEVICE_SAMPLE_RATE_PROPERTY_ADDRESS
        };
        self.get_property_data::<f64>(address).map_err(|e| e.into())
    }

    pub fn get_rate_range(
        &self,
        scope: &Scope
    ) -> Result<(f64, f64), Error> {
        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &INPUT_DEVICE_AVAILABLE_SAMPLE_RATE_PROPERTY_ADDRESS
        } else {
            &OUTPUT_DEVICE_AVAILABLE_SAMPLE_RATE_PROPERTY_ADDRESS
        };

        let ranges: Vec<AudioValueRange> = self.get_property_array(address)?;

        let mut max = f64::MIN;
        let mut min = f64::MAX;
        for range in ranges {
            max = get_max(max, range.mMaximum);
            min = get_min(min, range.mMinimum);
        }

        assert!(max >= min);
        Ok((min, max))
    }

    pub fn get_device_latency(
        &self,
        scope: &Scope
    ) -> Result<u32, Error> {
        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &INPUT_DEVICE_LATENCY_PROPERTY_ADDRESS
        } else {
            &OUTPUT_DEVICE_LATENCY_PROPERTY_ADDRESS
        };
        self.get_property_data::<u32>(address).map_err(|e| e.into())
    }

    pub fn get_stream_latency(
        &self,
        scope: &Scope
    ) -> Result<u32, Error> {
        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &INPUT_DEVICE_STREAMS_PROPERTY_ADDRESS
        } else {
            &OUTPUT_DEVICE_STREAMS_PROPERTY_ADDRESS
        };

        let streams: Vec<AudioStream> = self.get_property_array(address)?;

        // There may be several streams on a device. We use the first stream
        // to get the latency.
        // TODO: Is it correct?
        streams[0].get_latency(scope)
    }

    // TODO: Merge the get_device_latency and get_stream_latency as
    //       get_hardware_lantency
    // https://lists.apple.com/archives/coreaudio-api/2017/Jul/msg00035.html
    // pub fn get_hardware_lantency(
    //     &self,
    //     scope: &Scope
    // ) -> Result<u32, Error> {
    // }

    pub fn get_buffer_frame_size_range(
        &self,
        scope: &Scope
    ) -> Result<(f64, f64), Error> {
        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &INPUT_DEVICE_BUFFER_FRAME_SIZE_RANGE_PROPERTY_ADDRESS
        } else {
            &OUTPUT_DEVICE_BUFFER_FRAME_SIZE_RANGE_PROPERTY_ADDRESS
        };
        let range: AudioValueRange = self.get_property_data(address)?;
        Ok((range.mMinimum, range.mMaximum))
    }

    pub fn get_device_label(
        &self,
        scope: &Scope
    ) -> Result<String, Error> {
        // Some USB headset(e.g., Plantronics .Audio 628) fails to get its
        // source. In that case, we return device name instead.
        match self.get_device_source_name(scope) {
            Ok(name) => Ok(name),
            Err(Error::WrongScope) => Err(Error::WrongScope),
            Err(_) => self.get_device_name(),
        }
    }

    pub fn get_device_name(&self) -> Result<String, Error> {
        // The size of `StringRef` is same as the size of `CFStringRef`, so the
        // queried data of `CFStringRef` can be stored into the memory of a
        // `CFStringRef` variable directly.
        // If the calling fails, the StringRef::drop() will be called but
        // nothing will be released since StringRef::Default::default() is a
        // null string.
        let name: StringRef =
            self.get_property_data(&DEVICE_NAME_PROPERTY_ADDRESS)?;
        name.into_string().map_err(Error::ConversionFailed)
    }

    pub fn get_device_source_name(
        &self,
        scope: &Scope
    ) -> Result<String, Error> {
        let mut source: u32 = self.get_device_source(scope)?;
        let mut name: StringRef = StringRef::default(); // Create a null string.

        let mut translation: AudioValueTranslation = AudioValueTranslation {
            mInputData: &mut source as *mut u32 as *mut c_void,
            mInputDataSize: mem::size_of::<u32>() as u32,
            mOutputData: &mut name as *mut StringRef as *mut c_void,
            mOutputDataSize: mem::size_of::<StringRef>() as u32,
        };

        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &INPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS
        } else {
            &OUTPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS
        };

        self.get_property_data_with_ptr(address, &mut translation)?;
        name.into_string().map_err(Error::ConversionFailed)
    }

    fn get_device_source(
        &self,
        scope: &Scope
    ) -> Result<u32, Error> {
        if !self.in_scope(scope)? {
            return Err(Error::WrongScope);
        }

        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &INPUT_DEVICE_SOURCE_PROPERTY_ADDRESS
        } else {
            &OUTPUT_DEVICE_SOURCE_PROPERTY_ADDRESS
        };
        self.get_property_data::<u32>(address).map_err(|e| e.into())
    }

    pub fn in_scope(
        &self,
        scope: &Scope
    ) -> Result<bool, Error> {
        let streams = self.number_of_streams(scope)?;
        Ok(streams > 0)
    }

    fn number_of_streams(
        &self,
        scope: &Scope
    ) -> Result<usize, Error> {
        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &INPUT_DEVICE_STREAMS_PROPERTY_ADDRESS
        } else {
            &OUTPUT_DEVICE_STREAMS_PROPERTY_ADDRESS
        };
        let size = self.get_property_data_size(address)?;
        Ok(size / mem::size_of::<AudioStream>())
    }
}

impl Default for AudioObject {
    fn default() -> Self {
        AudioObject::new(kAudioObjectUnknown)
    }
}

impl GetObjectId for AudioObject {
    fn get_id(&self) -> AudioObjectID {
        self.0
    }
}

// TODO: Find a way to auto-implement Display for type that implements
//       GetObjectId.
impl fmt::Display for AudioObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_id())
    }
}

impl GetPropertyData for AudioObject {}
impl GetPropertyDataWithPtr for AudioObject {}
impl GetPropertyDataSize for AudioObject {}
impl GetPropertyVeriableSizedData for AudioObject {}
impl GetPropertyArray for AudioObject {}

// AudioStream
// ============================================================================
struct AudioStream(AudioStreamID); // AudioStreamID is AudioObjectID

impl AudioStream {
    fn get_latency(
        &self,
        scope: &Scope
    ) -> Result<u32, Error> {
        let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
            &INPUT_STREAM_LATENCY_PROPERTY_ADDRESS
        } else {
            &OUTPUT_STREAM_LATENCY_PROPERTY_ADDRESS
        };
        self.get_property_data(address)
    }
}

impl GetObjectId for AudioStream {
    fn get_id(&self) -> AudioObjectID {
        self.0
    }
}

impl GetPropertyData for AudioStream {}

// Utils
// ============================================================================
fn get_min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn get_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// Tests
// ============================================================================
#[cfg(test)]
mod test;