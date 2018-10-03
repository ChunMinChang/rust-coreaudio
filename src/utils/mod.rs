mod audio_objects;

use self::audio_objects::AudioSystemObject;
pub use self::audio_objects::{AudioObject, Scope, GetObjectId};

#[derive(Debug, PartialEq)]
pub enum Error {
    AudioObjects(audio_objects::Error),
}

impl From<audio_objects::Error> for Error {
    fn from(e: audio_objects::Error) -> Self {
        Error::AudioObjects(e)
    }
}

// Public APIs
// ============================================================================
// TODO: Use a static system_device to implement this APIs.
pub fn get_default_device(scope: &Scope) -> Result<AudioObject, Error> {
    let system_device = AudioSystemObject::new();
    system_device.get_default_device(scope).map_err(|e| e.into())
}

pub fn get_devices(scope: &Scope) -> Result<Vec<AudioObject>, Error> {
    let system_device = AudioSystemObject::new();
    system_device.get_devices(scope).map_err(|e| e.into())
}

pub fn get_all_devices() -> Result<Vec<AudioObject>, Error> {
    let system_device = AudioSystemObject::new();
    system_device.get_all_devices().map_err(|e| e.into())
}

pub fn set_default_device(
    device: &AudioObject,
    scope: &Scope
) -> Result<(), Error> {
    let system_device = AudioSystemObject::new();
    system_device.set_default_device(device, scope).map_err(|e| e.into())
}


// Tests
// ============================================================================
#[cfg(test)]
mod test;
