// Using `sys` as a prefix since we need to map the native error statuses like
// `kAudioHardwareBadObjectError` or `kAudioHardwareUnknownPropertyError` into
// our custom error type in `Error::from()`. If we directly use
// `kAudioHardwareBadObjectError` to compare with the given OSStatus variable,
// the `kAudioHardwareBadObjectError` in the `match` arm will be a new variable
// instead of the `kAudioHardwareBadObjectError` we expect. The following is an
// example:
//
// match status { // status' type is `OSStatus`.
//     kAudioHardwareBadObjectError => { ... }          // match to all status
//     kAudioHardwareUnknownPropertyError => { ... }    // unreachable pattern
//     ... => { ... }                                   // unreachable pattern
// }
//
// The `kAudioHardwareBadObjectError` is a new variable introduced in match
// block and it will match to all given `status`. It's not the
// `kAudioHardwareBadObjectError` defined in a `OSStatus` enum in CoreAudio
// as we expected.
extern crate coreaudio_sys as sys;

use std::fmt; // For fmt::{Debug, Formatter, Result}
use std::mem; // For mem::{size_of_val, size_of}
use std::os::raw::c_void;
use std::ptr; // For ptr::null()

// Using PartialEq for comparison.
#[derive(PartialEq)]
pub enum Error {
    BadObject,
    BadPropertySize,
    UnknownProperty,
    SizeIsZero,
}

impl From<sys::OSStatus> for Error {
    fn from(status: sys::OSStatus) -> Error {
        type BindgenOsstatusError = u32;
        fn to_bindgen_type(status: sys::OSStatus) -> BindgenOsstatusError {
            status as BindgenOsstatusError
        }

        match to_bindgen_type(status) {
            sys::kAudioHardwareBadObjectError => Error::BadObject,
            sys::kAudioHardwareBadPropertySizeError => Error::BadPropertySize,
            sys::kAudioHardwareUnknownPropertyError => Error::UnknownProperty,
            s => panic!("Unknown status: {}", s),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Error::BadObject => "BadObject: The AudioObjectID passed to the function doesn't map to a valid AudioObject.",
            Error::BadPropertySize => "BadPropertySize: An improperly sized buffer was provided when accessing the data of a property.",
            Error::UnknownProperty => "UnknownProperty: The AudioObject doesn't know about the property at the given address.",
            Error::SizeIsZero => "SizeIsZero: The size of data mapping to the given id and address is zero.",
        };
        write!(f, "{}", printable)
    }
}

// Public APIs
// ============================================================================
pub fn has_property(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress
) -> bool {
    audio_object_has_property(id, address)
}

pub fn get_property_data<T: Default>(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
) -> Result<T, Error> {
    // assert!(id != sys::kAudioObjectUnknown, "Bad AudioObjectID!");
    // When this function fails(returns Err), the `data` will be dropped and
    // T::drop() will be fired.
    let mut data: T = Default::default();
    get_property_data_with_ptr(id, address, &mut data)?;
    Ok(data)
}

pub fn get_property_data_with_ptr<T>(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
    data: &mut T,
) -> Result<(), Error> {
    // assert!(id != sys::kAudioObjectUnknown, "Bad AudioObjectID!");
    let mut size = mem::size_of_val(data);
    assert_eq!(size, get_property_data_size(id, address)?);
    let status = audio_object_get_property_data::<T>(id, address, &mut size, data);
    convert_to_result(status)
}

pub fn get_property_data_size(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
) -> Result<usize, Error> {
    // assert!(id != sys::kAudioObjectUnknown, "Bad AudioObjectID!");
    let mut size = 0;
    let status = audio_object_get_property_data_size(id, address, &mut size);
    convert_to_result(status)?;
    Ok(size)
}

pub fn get_property_array<T>(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
) -> Result<Vec<T>, Error> {
    // assert!(id != sys::kAudioObjectUnknown, "Bad AudioObjectID!");
    let mut size = non_empty_size(get_property_data_size(id, address))?;
    let mut array = allocate_array::<T>(size);
    let status = audio_object_get_property_data::<T>(id, address, &mut size, array.as_mut_ptr());
    convert_to_result(status)?;
    Ok(array)
}

pub fn get_property_variable_sized_data<'a, T>(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
) -> Result<&'a T, Error> {
    // assert!(id != sys::kAudioObjectUnknown, "Bad AudioObjectID!");
    let mut size = non_empty_size(get_property_data_size(id, address))?;
    let mut buffer = allocate_array::<u8>(size);
    let ptr = buffer.as_mut_ptr() as *mut T;
    let status = audio_object_get_property_data::<T>(id, address, &mut size, ptr);
    convert_to_result(status)?;
    let data = unsafe { &(*ptr) };
    Ok(data)
}

pub fn set_property_data<T>(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
    data: &T,
) -> Result<(), Error> {
    // assert!(id != sys::kAudioObjectUnknown, "Bad AudioObjectID!");
    let size = mem::size_of::<T>();
    let status = audio_object_set_property_data::<T>(id, address, size, data);
    convert_to_result(status)
}

// Private APIs
// ============================================================================
fn allocate_array<T>(size: usize) -> Vec<T> {
    let elements = size / mem::size_of::<T>();
    let mut buffer = Vec::<T>::with_capacity(elements);
    unsafe {
        buffer.set_len(elements);
    }
    buffer
}

fn non_empty_size(result: Result<usize, Error>) -> Result<usize, Error> {
    let value = result?;
    if value > 0 {
        Ok(value)
    } else {
        Err(Error::SizeIsZero)
    }
}

fn convert_to_result(status: sys::OSStatus) -> Result<(), Error> {
    match status {
        0 => Ok(()), // 0 is sys::kAudioHardwareNoError.
        s => Err(s.into()),
    }
}

// Wrappers for native platform APIs
// ----------------------------------------------------------------------------
fn audio_object_has_property(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
) -> bool {
    unsafe {
        sys::AudioObjectHasProperty(
            id,
            address,
        ) != 0
    }
}

fn audio_object_get_property_data<T>(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
    size: *mut usize,
    data: *mut T,
) -> sys::OSStatus {
    unsafe {
        sys::AudioObjectGetPropertyData(
            id,
            address, // as `*const AudioObjectPropertyAddress` automatically.
            0,
            ptr::null(),
            size as *mut sys::UInt32, // Cast raw usize pointer to raw u32 pointer.
            data as *mut c_void, // Cast raw T pointer to void pointer.
        )
    }
}

fn audio_object_get_property_data_size(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
    size: *mut usize,
) -> sys::OSStatus {
    unsafe {
        sys::AudioObjectGetPropertyDataSize(
            id,
            address, // as `*const AudioObjectPropertyAddress` automatically.
            0,
            ptr::null(),
            size as *mut sys::UInt32, // Cast raw usize pointer to raw u32 pointer.
        )
    }
}

fn audio_object_set_property_data<T>(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
    size: usize,
    data: *const T,
) -> sys::OSStatus {
    unsafe {
        sys::AudioObjectSetPropertyData(
            id,
            address, // as `*const AudioObjectPropertyAddress` automatically.
            0,
            ptr::null(),
            size as sys::UInt32, // Cast usize variable to raw u32 variable.
            data as *const c_void, // Cast raw T pointer to void pointer.
        )
    }
}

// Tests
// ============================================================================
#[cfg(test)]
mod test;
