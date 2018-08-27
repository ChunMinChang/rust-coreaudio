extern crate core_foundation_sys;
extern crate coreaudio_sys;

mod audio_object;
mod string_helper;

use self::core_foundation_sys::string::CFStringRef;
use self::coreaudio_sys::{
    kAudioObjectPropertyName,
    kAudioHardwarePropertyDevices,
    kAudioHardwarePropertyDefaultInputDevice,
    kAudioHardwarePropertyDefaultOutputDevice,
    kAudioDevicePropertyStreams,
    kAudioDevicePropertyDataSource,
    kAudioDevicePropertyDataSourceNameForIDCFString,
    kAudioObjectPropertyScopeInput,
    kAudioObjectPropertyScopeOutput,
    kAudioObjectPropertyScopeGlobal,
    kAudioObjectPropertyElementMaster,
    AudioObjectPropertyAddress,
    AudioObjectID,
    kAudioObjectSystemObject,   // AudioObjectID
    kAudioObjectUnknown,        // AudioObjectID
    AudioStreamID,              // AudioObjectID
    AudioValueTranslation,
};
use std::fmt; // For fmt::{Debug, Formatter, Result}
use std::mem; // For mem::{uninitialized(), size_of()}
use std::os::raw::c_void;
use std::ptr; // For ptr::null()

const DEVICE_NAME_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioObjectPropertyName,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

const DEVICE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDevices,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

const DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDefaultInputDevice,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

const DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDefaultOutputDevice,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

const INPUT_DEVICE_STREAMS_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyStreams,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

const OUTPUT_DEVICE_STREAMS_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyStreams,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

const INPUT_DEVICE_SOURCE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDataSource,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

const OUTPUT_DEVICE_SOURCE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDataSource,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

const INPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDataSourceNameForIDCFString,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

const OUTPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDataSourceNameForIDCFString,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

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
    ConversionFailed(string_helper::Error),
    InvalidParameters(audio_object::Error),
    NoDeviceFound,
    SetSameDevice,
    WrongScope,
}

// To convert an audio_object::Error to a Error.
impl From<audio_object::Error> for Error {
    fn from(e: audio_object::Error) -> Error {
        Error::InvalidParameters(e)
    }
}

// To convert an string_helper::Error to a Error.
impl From<string_helper::Error> for Error {
    fn from(e: string_helper::Error) -> Error {
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

// Public APIs
// ========================================================================
pub fn get_default_device_id(scope: &Scope) -> Result<AudioObjectID, Error> {
    let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
        &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS
    } else {
        &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS
    };
    let id: AudioObjectID =
        audio_object::get_property_data::<AudioObjectID>(kAudioObjectSystemObject, address)?;
    // `id` will be kAudioObjectUnknown when there is no valid device.
    // Return `NoDeviceFound` error in this case.
    if id == kAudioObjectUnknown {
        Err(Error::NoDeviceFound)
    } else {
        Ok(id)
    }
}

pub fn in_scope(id: AudioObjectID, scope: &Scope) -> Result<bool, Error> {
    let streams = number_of_streams(id, scope)?;
    Ok(streams > 0)
}

// Apple has no API to get input-only or output-only devices. To do that, we
// need to get all the devices first ans then check if they are input or output
// ony by one.
pub fn get_device_ids(scope: &Scope) -> Result<Vec<AudioObjectID>, Error> {
    let mut devices: Vec<AudioObjectID> = get_all_device_ids()?;
    // It's ok to call `unwrap()` here since all the `AudioObjectID` values
    // in `devices` are valid.
    devices.retain(|&device| in_scope(device, scope).unwrap());
    Ok(devices)
}

pub fn get_all_device_ids() -> Result<Vec<AudioObjectID>, Error> {
    let ids: Vec<AudioObjectID> = audio_object::get_property_array::<AudioObjectID>(
        kAudioObjectSystemObject,
        &DEVICE_PROPERTY_ADDRESS,
    )?;
    Ok(ids)
}

pub fn get_device_label(id: AudioObjectID, scope: &Scope) -> Result<String, Error> {
    match get_device_source_name(id, scope) {
        Ok(name) => Ok(name),
        Err(Error::WrongScope) => Err(Error::WrongScope),
        Err(_) => get_device_name(id),
    }
}

pub fn get_device_name(id: AudioObjectID) -> Result<String, Error> {
    let name: CFStringRef =
        audio_object::get_property_data::<CFStringRef>(id, &DEVICE_NAME_PROPERTY_ADDRESS)?;
    string_helper::to_string(name).map_err(Error::ConversionFailed)
    // TODO: The memory pointed by `name` will be free in to_string(...).
    //       Find a way to move `name` to prevent it from being a dangling
    //       pointer.
}

pub fn get_device_source_name(id: AudioObjectID, scope: &Scope) -> Result<String, Error> {
    let mut source: u32 = get_device_source(id, scope)?;
    let mut name: CFStringRef = ptr::null();

    let mut translation: AudioValueTranslation = AudioValueTranslation {
        mInputData: &mut source as *mut u32 as *mut c_void,
        mInputDataSize: mem::size_of::<u32>() as u32,
        mOutputData: &mut name as *mut CFStringRef as *mut c_void,
        mOutputDataSize: mem::size_of::<CFStringRef>() as u32,
    };

    let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
        &INPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS
    } else {
        &OUTPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS
    };
    audio_object::get_property_data_with_ptr(id, address, &mut translation)?;
    string_helper::to_string(name).map_err(Error::ConversionFailed)
    // TODO: The memory pointed by `name` will be free in to_string(...).
    //       Find a way to move `name` to prevent it from being a dangling
    //       pointer.
}

pub fn set_default_device(id: AudioObjectID, scope: &Scope) -> Result<(), Error> {
    // Surprisingly it's ok to set
    //   1. a unknown device
    //   2. a non-input/non-output device
    //   3. the current default input/output device
    // as the new default input/output device by apple's API.
    // We need to check the above things by ourselves.
    if !in_scope(id, scope)? {
        return Err(Error::WrongScope);
    }
    let default_id = get_default_device_id(scope)?;
    if id == default_id {
        return Err(Error::SetSameDevice);
    }
    let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
        &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS
    } else {
        &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS
    };
    audio_object::set_property_data(kAudioObjectSystemObject, address, &id)?;
    Ok(())
}

// Private APIs
// ========================================================================
fn get_device_source(id: AudioObjectID, scope: &Scope) -> Result<u32, Error> {
    if !in_scope(id, scope)? {
        return Err(Error::WrongScope);
    }

    let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
        &INPUT_DEVICE_SOURCE_PROPERTY_ADDRESS
    } else {
        &OUTPUT_DEVICE_SOURCE_PROPERTY_ADDRESS
    };
    let source = audio_object::get_property_data::<u32>(id, address)?;
    Ok(source)
}

fn number_of_streams(id: AudioObjectID, scope: &Scope) -> Result<usize, Error> {
    let address: &AudioObjectPropertyAddress = if scope == &Scope::Input {
        &INPUT_DEVICE_STREAMS_PROPERTY_ADDRESS
    } else {
        &OUTPUT_DEVICE_STREAMS_PROPERTY_ADDRESS
    };
    let size = audio_object::get_property_data_size(id, address)?;
    Ok(size / mem::size_of::<AudioStreamID>())
}

#[cfg(test)]
mod test;
