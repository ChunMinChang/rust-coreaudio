// Use `sys` as a prefix since we need to map the native error statuses like
// `kAudioHardwareBadObjectError` or `kAudioHardwareUnknownPropertyError` into
// our custom error type in `status_to_error()`. If we directly use
// `kAudioHardwareBadObjectError` to compare with the given OSStatus variable,
// the `kAudioHardwareBadObjectError` in the `match` arm will be a new variable
// instead of the `kAudioHardwareBadObjectError` we expect. The following is an
// example:
//
// match status { // status' type is `OSStatus`.
//     kAudioHardwareBadObjectError => { ... }          // match to all status
//     kAudioHardwareUnknownPropertyError => { ... }    // unreachable pattern
//     ... => => { ... }                                // unreachable pattern
// }
//
// The `kAudioHardwareBadObjectError` is a new variable introducec in match
// block and it will match to all given `status`. It's not the
// `kAudioHardwareBadObjectError` defined in a `OSStatus` enum in CoreAudio.
extern crate coreaudio_sys as sys;

use std::fmt;             // For fmt::{Debug, Formatter, Result}
use std::mem;             // For mem::{uninitialized(), size_of()}
use std::os::raw::c_void; // For `void*`
use std::ptr;             // For ptr::null()

// Using PartialEq for comparison.
#[derive(PartialEq)]
pub enum Error {
    BadObject,
    UnknownProperty,
    SizeIsZero,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Error::BadObject => "BadObject: The AudioObjectID passed to the function doesn't map to a valid AudioObject.",
            Error::UnknownProperty => "UnknownProperty: The AudioObject doesn't know about the property at the given address.",
            Error::SizeIsZero => "SizeIsZero: The size of data mapping to the given id and address is zero.",
        };
        write!(f, "{}", printable)
    }
}

pub fn get_property_data<T>(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
) -> Result<T, Error> {
    // assert!(id != sys::kAudioObjectUnknown, "Bad AudioObjectID!");
    // Use `mem::uninitialized()` to bypasses memory-initialization checks.
    let mut data: T = unsafe { mem::uninitialized() };
    get_property_data_with_ptr(id, address, &mut data)?;
    Ok(data)
}

pub fn get_property_data_with_ptr<T>(
    id: sys::AudioObjectID,
    address: &sys::AudioObjectPropertyAddress,
    data: &mut T,
) -> Result<(), Error> {
    // assert!(id != sys::kAudioObjectUnknown, "Bad AudioObjectID!");
    let mut size = mem::size_of::<T>();
    debug_assert_eq!(size, get_property_data_size(id, address)?);
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
) -> Result<Vec<T>, Error>
where
    T: Sized,
{
    // assert!(id != sys::kAudioObjectUnknown, "Bad AudioObjectID!");
    let mut size = non_empty_size(get_property_data_size(id, address))?;
    let elements = size / mem::size_of::<T>();
    let mut array = Vec::<T>::with_capacity(elements);
    unsafe {
        array.set_len(elements);
    }
    let status = audio_object_get_property_data::<T>(id, address, &mut size, array.as_mut_ptr());
    convert_to_result(status)?;
    Ok(array)
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

fn non_empty_size(result: Result<usize, Error>) -> Result<usize, Error> {
    let value = result?;
    if value > 0 {
        Ok(value)
    } else {
        Err(Error::SizeIsZero)
    }
}

fn convert_to_result(status: sys::OSStatus) -> Result<(), Error> {
    match to_bindgen_type(status) {
        sys::kAudioHardwareNoError => Ok(()),
        e => Err(status_to_error(e)),
    }
}

fn status_to_error(status: BindgenOsstatus) -> Error {
    match status {
        sys::kAudioHardwareBadObjectError => Error::BadObject,
        sys::kAudioHardwareUnknownPropertyError => Error::UnknownProperty,
        error => panic!("Unknown error: {}", error),
    }
}

type BindgenOsstatus = u32;
fn to_bindgen_type(status: sys::OSStatus) -> BindgenOsstatus {
    status as BindgenOsstatus
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
            size as *mut u32,    // Cast raw usize pointer to raw u32 pointer.
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
            size as *mut u32, // Cast raw usize pointer to raw u32 pointer.
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
            size as u32,           // Cast usize variable to raw u32 variable.
            data as *const c_void, // Cast raw T pointer to void pointer.
        )
    }
}

#[cfg(test)]
mod test;
