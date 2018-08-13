extern crate coreaudio_sys as sys;
extern crate core_foundation_sys as cf; // Force CoreFundation being built within project.

use std::ffi::CStr; // For CStr
use std::fmt; // For fmt::Display
use std::mem; // For mem::uninitialized(), mem::size_of
use std::os::raw::{c_void, c_char}; // For `void*`
use std::ptr; // For ptr::null()

const DEVICE_NAME_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress =
    sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioObjectPropertyName,
        mScope: sys::kAudioObjectPropertyScopeGlobal,
        mElement: sys::kAudioObjectPropertyElementMaster,
    };

const DEVICE_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress =
    sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioHardwarePropertyDevices,
        mScope: sys::kAudioObjectPropertyScopeGlobal,
        mElement: sys::kAudioObjectPropertyElementMaster,
    };

const DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress =
    sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioHardwarePropertyDefaultInputDevice,
        mScope: sys::kAudioObjectPropertyScopeGlobal,
        mElement: sys::kAudioObjectPropertyElementMaster,
    };

const DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress =
    sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioHardwarePropertyDefaultOutputDevice,
        mScope: sys::kAudioObjectPropertyScopeGlobal,
        mElement: sys::kAudioObjectPropertyElementMaster,
    };

const INPUT_DEVICE_STREAMS_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress =
    sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioDevicePropertyStreams,
        mScope: sys::kAudioObjectPropertyScopeInput,
        mElement: sys::kAudioObjectPropertyElementMaster,
    };

const OUTPUT_DEVICE_STREAMS_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress =
    sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioDevicePropertyStreams,
        mScope: sys::kAudioObjectPropertyScopeOutput,
        mElement: sys::kAudioObjectPropertyElementMaster,
    };

const INPUT_DEVICE_SOURCE_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress =
    sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioDevicePropertyDataSource,
        mScope: sys::kAudioObjectPropertyScopeInput,
        mElement: sys::kAudioObjectPropertyElementMaster,
    };

const OUTPUT_DEVICE_SOURCE_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress =
    sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioDevicePropertyDataSource,
        mScope: sys::kAudioObjectPropertyScopeOutput,
        mElement: sys::kAudioObjectPropertyElementMaster,
    };

const INPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress =
    sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioDevicePropertyDataSourceNameForIDCFString,
        mScope: sys::kAudioObjectPropertyScopeInput,
        mElement: sys::kAudioObjectPropertyElementMaster
    };

const OUTPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress =
    sys::AudioObjectPropertyAddress {
        mSelector: sys::kAudioDevicePropertyDataSourceNameForIDCFString,
        mScope: sys::kAudioObjectPropertyScopeOutput,
        mElement: sys::kAudioObjectPropertyElementMaster
    };

// #[repr(C)] // Specify data layout in the same way as C does.
#[derive(PartialEq)] // Enable comparison.
pub enum Scope {
    Input,
    Output,
}

// #[repr(C)] // Specify data layout in the same way as C does.
#[derive(Debug, PartialEq)] // Using Debug for std::fmt::Debug.
pub enum Error {
    // NoError,
    NotFound,
    InvalidParameters,
    ConversionFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            // Error::NoError => "Everything is fine",
            Error::NotFound => "Data not found",
            Error::InvalidParameters => "Invalid parameters",
            Error::ConversionFailed => "Conversion Failed",
        };
        write!(f, "{}", printable)
    }
}

// Public APIs
// ========================================================================
pub fn get_default_device_id(scope: &Scope) -> Result<sys::AudioObjectID, Error> {
    let address: &sys::AudioObjectPropertyAddress = if scope == &Scope::Input {
        &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS
    } else {
        &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS
    };
    let id: sys::AudioObjectID = get_property_data::<sys::AudioObjectID>(
        &sys::kAudioObjectSystemObject, address)?;
    if id == sys::kAudioObjectUnknown {
        Err(Error::NotFound)
    } else {
        Ok(id)
    }
}

pub fn in_scope(id: &sys::AudioObjectID, scope: &Scope) -> Result<bool, Error> {
    let streams = number_of_streams(id, scope)?;
    Ok(streams > 0)
}

pub fn get_device_ids(scope: &Scope) -> Result<Vec<sys::AudioObjectID>, Error> {
    // let mut devices: Vec<sys::AudioObjectID> = get_all_device_ids()?;
    // devices.retain(|&device| in_scope(&device, scope).unwrap());
    // Ok(devices)
    let devices: Vec<sys::AudioObjectID> = get_all_device_ids()?;
    let mut devices_in_scope: Vec<sys::AudioObjectID> = Vec::new();
    for device in &devices {
        if in_scope(&device, scope)? {
            devices_in_scope.push(device.clone());
        }
    }
    Ok(devices_in_scope)
}


pub fn get_all_device_ids() -> Result<Vec<sys::AudioObjectID>, Error> {
    let ids: Vec<sys::AudioObjectID> = get_property_array::<sys::AudioObjectID>(
        &sys::kAudioObjectSystemObject, &DEVICE_PROPERTY_ADDRESS)?;
    Ok(ids)
}

pub fn get_device_label(id: &sys::AudioObjectID, scope: &Scope) -> Result<String, Error> {
    match get_device_source_name(id, scope) {
        Ok(name) => { Ok(name) },
        Err(_) => {
            let id_in_scope = in_scope(id, scope)?; // Check if the id is in scope.
            if id_in_scope { get_device_name(id) } else { Err(Error::NotFound) }
        }
    }
}

pub fn get_device_name(id: &sys::AudioObjectID) -> Result<String, Error> {
    let name: sys::CFStringRef = get_property_data::<sys::CFStringRef>(
        id, &DEVICE_NAME_PROPERTY_ADDRESS)?;
    to_string(name)
}

pub fn get_device_source_name(id: &sys::AudioObjectID, scope: &Scope) -> Result<String, Error> {
    let mut source: u32 = get_device_source(id, scope)?;
    let mut name: sys::CFStringRef = ptr::null();

    let mut translation: sys::AudioValueTranslation = sys::AudioValueTranslation {
        mInputData: &mut source as *mut u32 as *mut c_void,
        mInputDataSize: mem::size_of::<u32>() as u32,
        mOutputData: &mut name as *mut sys::CFStringRef as *mut c_void,
        mOutputDataSize: mem::size_of::<sys::CFStringRef>() as u32,
    };

    let address: &sys::AudioObjectPropertyAddress = if scope == &Scope::Input {
        &INPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS
    } else {
        &OUTPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS
    };
    get_property_data_with_ptr(id, address, &mut translation)?;
    to_string(name)
}

// Private APIs
// ========================================================================
fn get_device_source(id: &sys::AudioObjectID, scope: &Scope) -> Result<u32, Error> {
    let address: &sys::AudioObjectPropertyAddress = if scope == &Scope::Input {
        &INPUT_DEVICE_SOURCE_PROPERTY_ADDRESS
    } else {
        &OUTPUT_DEVICE_SOURCE_PROPERTY_ADDRESS
    };
    get_property_data::<u32>(id, address)
}

fn number_of_streams(id: &sys::AudioObjectID, scope: &Scope) -> Result<usize, Error> {
    let address: &sys::AudioObjectPropertyAddress = if scope == &Scope::Input {
        &INPUT_DEVICE_STREAMS_PROPERTY_ADDRESS
    } else {
        &OUTPUT_DEVICE_STREAMS_PROPERTY_ADDRESS
    };
    let size = get_property_data_size(id, address)?;
    Ok(size / mem::size_of::<sys::AudioStreamID>())
}

fn to_string(cf_string_ref: sys::CFStringRef) -> Result<String, Error> {
    assert!(!cf_string_ref.is_null());
    let buffer:Vec<c_char> = get_btye_array(cf_string_ref)?;
    btye_array_to_string(buffer)
}

fn get_btye_array(cf_string_ref: sys::CFStringRef) -> Result<Vec<c_char>, Error> {
    let length: sys::CFIndex = unsafe { sys::CFStringGetLength(cf_string_ref) };
    if length <= 0 {
        return Err(Error::ConversionFailed);
    }
    let size: sys::CFIndex = unsafe { sys::CFStringGetMaximumSizeForEncoding(
        length, sys::kCFStringEncodingUTF8) } + 1;
    let mut buffer = Vec::<c_char>::with_capacity(size as usize);
    let success: bool = unsafe {
        buffer.set_len(size as usize);
        let result: sys::Boolean = sys::CFStringGetCString(
            cf_string_ref, buffer.as_mut_ptr(), size, sys::kCFStringEncodingUTF8);
        sys::CFRelease(cf_string_ref as *mut c_void);
        result != 0 // sys::Boolean is u8, so compare with 0 to get bool.
    };
    if success { Ok(buffer) } else { return Err(Error::ConversionFailed); }
}

fn btye_array_to_string(mut buffer: Vec<c_char>) -> Result<String, Error> {
    // CStr::from_ptr will call strlen to trim the array first:
    // https://doc.rust-lang.org/src/std/ffi/c_str.rs.html#935-939
    let c_str: &CStr = unsafe { CStr::from_ptr(buffer.as_mut_ptr()) };
    let str_slice: &str = match c_str.to_str() {
        Ok(slice) => slice,
        Err(_) => return Err(Error::ConversionFailed),
    };
    let str_buf: String = str_slice.to_string();
    Ok(str_buf)
}

// fn get_property_data<T> (
//     id: &sys::AudioObjectID,
//     address: &sys::AudioObjectPropertyAddress,
// ) -> Result<T, Error> {
//     assert!(id != &sys::kAudioObjectUnknown, "Invalid AudioObjectID!");
//     let mut size = mem::size_of::<T>();
//     // Use `mem::uninitialized()` to bypasses memory-initialization checks.
//     let mut data: T = unsafe { mem::uninitialized() };
//     let status = audio_object_get_property_data::<T>(
//         id, address, &mut size, &mut data);
//     convert_to_result(status)?;
//     Ok(data)
// }

fn get_property_data<T> (
    id: &sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
) -> Result<T, Error> {
    assert!(id != &sys::kAudioObjectUnknown, "Invalid AudioObjectID!");
    // Use `mem::uninitialized()` to bypasses memory-initialization checks.
    let mut data: T = unsafe { mem::uninitialized() };
    get_property_data_with_ptr(id, address, &mut data)?;
    Ok(data)
}

fn get_property_data_with_ptr<T> (
    id: &sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
    data: &mut T,
) -> Result<(), Error> {
    assert!(id != &sys::kAudioObjectUnknown, "Invalid AudioObjectID!");
    let mut size = mem::size_of::<T>();
    // debug_assert_eq!(size, get_property_data_size(id, address)?);
    let status = audio_object_get_property_data::<T>(
        id, address, &mut size, data);
    convert_to_result(status)
}

fn get_property_data_size (
    id: &sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
) -> Result<usize, Error> {
    assert!(id != &sys::kAudioObjectUnknown, "Invalid AudioObjectID!");
    let mut size  = 0;
    let status = audio_object_get_property_data_size(id, address, &mut size);
    convert_to_result(status)?;
    Ok(size)
}

fn get_property_array<T> (
    id: &sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
) -> Result<Vec<T>, Error>
where
    T: Sized {
    assert!(id != &sys::kAudioObjectUnknown, "Invalid AudioObjectID!");
    let mut size = non_empty_size(get_property_data_size(id, address))?;
    let elements = size / mem::size_of::<T>();
    let mut array = Vec::<T>::with_capacity(elements);
    unsafe { array.set_len(elements); }
    let status = audio_object_get_property_data::<T>(
        id, address, &mut size, array.as_mut_ptr());
    convert_to_result(status)?;
    Ok(array)
}

fn non_empty_size(result: Result<usize, Error>) -> Result<usize, Error> {
    let value = result?;
    if value > 0 { Ok(value) } else { Err(Error::NotFound) }
}

fn convert_to_result(status: sys::OSStatus) -> Result<(), Error> {
    match to_bindgen_type(status) {
        sys::kAudioHardwareNoError => Ok(()),
        e => Err(status_to_error(e)),
    }
}

fn status_to_error(status: BindgenOsstatus) -> Error {
    match status {
        sys::kAudioHardwareBadObjectError => Error::InvalidParameters,
        sys::kAudioHardwareUnknownPropertyError => Error::NotFound,
        error => panic!("Unknown error: {}", error),
    }
}

type BindgenOsstatus = u32;
fn to_bindgen_type(status: sys::OSStatus) -> BindgenOsstatus {
    status as BindgenOsstatus
}

fn audio_object_get_property_data<T>(
    id: &sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
    size: *mut usize,
    data: *mut T,
) -> sys::OSStatus {
    unsafe {
        sys::AudioObjectGetPropertyData(
            *id,
            address, // as `*const AudioObjectPropertyAddress` automatically.
            0,
            ptr::null(),
            size as *mut u32, // Cast raw usize pointer to raw u32 pointer.
            data as *mut c_void, // Cast raw T pointer to void pointer.
        )
    }
}

fn audio_object_get_property_data_size(
    id: &sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
    size: *mut usize,
) -> sys::OSStatus {
    unsafe {
        sys::AudioObjectGetPropertyDataSize(
            *id,
            address, // as `*const AudioObjectPropertyAddress` automatically.
            0,
            ptr::null(),
            size as *mut u32, // Cast raw usize pointer to raw u32 pointer.
        )
    }
}

#[cfg(test)]
mod test;