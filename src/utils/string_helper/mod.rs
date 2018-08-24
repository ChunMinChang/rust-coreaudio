extern crate core_foundation_sys;
extern crate coreaudio_sys;

use self::core_foundation_sys::base::{Boolean, CFIndex, CFRange, CFRelease};
use self::core_foundation_sys::string::{kCFStringEncodingUTF8, CFStringGetBytes, CFStringGetLength, CFStringRef};
use std::fmt; // For fmt::{Debug, Formatter, Result}
use std::os::raw::c_void;
use std::ptr; // For ptr::null()
use std::str::Utf8Error;

// TODO: Put the reason of the failure inside the error state
//       (e.g., why CFStringGetBytes fail? the `converted_chars` is 0 or
//        other reason?).
// Using PartialEq for comparison.
#[derive(PartialEq)]
pub enum Error {
    FailToGetBytes,
    LengthIsZero,
    Utf8(Utf8Error),
}

// To convert an string_helper::Error to a Error.
impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Error {
        Error::Utf8(e)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Error::FailToGetBytes => {
                "Fail to get bytes from CFStringRef by given encoding".to_string()
            }
            Error::LengthIsZero => "String length is zero.".to_string(),
            Error::Utf8(e) => format!("Fail to convert a vec into UTF8 string: {:?}.", e),
        };
        write!(f, "{}", printable)
    }
}

pub fn to_string(string_ref: CFStringRef) -> Result<String, Error> {
    assert!(!string_ref.is_null());
    let buffer = get_btye_array(string_ref)?;
    btye_array_to_string(buffer)
}

fn get_btye_array(string_ref: CFStringRef) -> Result<Vec<u8>, Error> {
    // First, get the size of the buffer ought to be.
    let length: CFIndex = unsafe { CFStringGetLength(string_ref) };
    if length <= 0 {
        unsafe { CFRelease(string_ref as *mut c_void) };
        return Err(Error::LengthIsZero);
    }
    let range: CFRange = CFRange {
        location: 0,
        length,
    };
    let mut size: CFIndex = 0;
    let mut converted_chars: CFIndex = unsafe {
        CFStringGetBytes(
            string_ref,
            range,
            kCFStringEncodingUTF8,
            0,
            false as Boolean,
            ptr::null_mut() as *mut u8,
            0,
            &mut size,
        )
    };

    if converted_chars <= 0 || size <= 0 {
        unsafe { CFRelease(string_ref as *mut c_void) };
        return Err(Error::FailToGetBytes);
    }
    // TODO: Figure out if converted_chars = size = length in any case.
    //       Change the condition above if it's always true.
    assert_eq!(size, length);
    assert_eq!(converted_chars, length);

    // Then, allocate the buffer with the required size and actually copy data into it.
    let mut buffer = vec![b'\x00'; size as usize];
    converted_chars = unsafe {
        CFStringGetBytes(
            string_ref,
            range,
            kCFStringEncodingUTF8,
            0,
            false as Boolean,
            buffer.as_mut_ptr(),
            size,
            ptr::null_mut() as* mut CFIndex,
        )
    };
    unsafe { CFRelease(string_ref as *mut c_void) };
    if converted_chars <= 0 {
        return Err(Error::FailToGetBytes);
    }
    // TODO: Figure out if converted_chars = size( = length) in any case.
    //       Change the condition above if it's always true.
    assert_eq!(converted_chars, size);
    Ok(buffer)
}

fn btye_array_to_string(buffer: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(buffer).map_err(|e| Error::Utf8(e.utf8_error()))
}

#[cfg(test)]
mod test;
