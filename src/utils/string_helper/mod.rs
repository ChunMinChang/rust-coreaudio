extern crate core_foundation_sys;
extern crate coreaudio_sys;

use self::core_foundation_sys::base::{Boolean, CFIndex, CFRelease};
use self::core_foundation_sys::string::{kCFStringEncodingUTF8, CFStringGetCString, CFStringGetLength, CFStringRef};
// TODO: Replace `CFStringGetMaximumSizeForEncoding` by `CFRange` and
//       `CFStringGetBytes` in CoreFoundation later.
use self::coreaudio_sys::CFStringGetMaximumSizeForEncoding;
use std::ffi::CStr;
use std::fmt; // For fmt::{Debug, Formatter, Result}
use std::os::raw::{c_char, c_void};
use std::str::Utf8Error;

// Using PartialEq for comparison.
#[derive(PartialEq)]
pub enum Error {
    FailToGetCString,
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
            Error::FailToGetCString => "Fail to get C String from CFStringRef by given encoding: Maybe the provided buffer is too small.".to_string(),
            Error::LengthIsZero => "String length is zero.".to_string(),
            Error::Utf8(e) => format!("Fail to convert to UTF8 string: {:?}.", e),
        };
        write!(f, "{}", printable)
    }
}

// TODO: Move string conversion to another module maybe.
pub fn to_string(string_ref: CFStringRef) -> Result<String, Error> {
    assert!(!string_ref.is_null());
    let buffer: Vec<c_char> = get_btye_array(string_ref)?;
    btye_array_to_string(buffer)
}

fn get_btye_array(string_ref: CFStringRef) -> Result<Vec<c_char>, Error> {
    let length: CFIndex = unsafe { CFStringGetLength(string_ref) };
    if length <= 0 {
        return Err(Error::LengthIsZero);
    }
    let size: CFIndex =
        unsafe { CFStringGetMaximumSizeForEncoding(length, kCFStringEncodingUTF8) } + 1;
    let mut buffer = Vec::<c_char>::with_capacity(size as usize);
    let success: bool = unsafe {
        buffer.set_len(size as usize);
        let result: Boolean =
            CFStringGetCString(string_ref, buffer.as_mut_ptr(), size, kCFStringEncodingUTF8);
        CFRelease(string_ref as *mut c_void);
        result != 0 // Boolean is u8. Returing a bool by comparing with 0.
    };
    if success {
        Ok(buffer)
    } else {
        Err(Error::FailToGetCString)
    }
}

fn btye_array_to_string(mut buffer: Vec<c_char>) -> Result<String, Error> {
    // CStr::from_ptr will call strlen to trim the array first. See:
    // https://doc.rust-lang.org/src/std/ffi/c_str.rs.html#935-939
    let c_str: &CStr = unsafe { CStr::from_ptr(buffer.as_mut_ptr()) };
    let str_slice: &str = c_str.to_str()?;
    let str_buf: String = str_slice.to_string();
    Ok(str_buf)
}

#[cfg(test)]
mod test;
