use super::*;
use utils::get_default_device;

// AudioSystemObject
// ============================================================================
// Skip for now ...

// AudioObject
// ============================================================================
// is_valid
// --------------------------
#[test]
fn test_is_valid() {
  let unknown_device = AudioObject::new(kAudioObjectUnknown);
  assert!(!unknown_device.is_valid());
}

// get_device_label
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_get_device_label_with_invalid_id() {
    let unknown_device = AudioObject::new(kAudioObjectUnknown);
    assert_eq!(
        unknown_device.get_device_label(&Scope::Input).unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
    assert_eq!(
        unknown_device.get_device_label(&Scope::Output).unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
}

#[test]
fn test_get_device_label_with_invalid_scope() {
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
        let is_output = device.in_scope(&Scope::Output).unwrap();
        if !is_output {
            assert_eq!(
                device.get_device_label(&Scope::Output).unwrap_err(),
                Error::WrongScope
            );
        }
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        let is_input = device.in_scope(&Scope::Input).unwrap();
        if !is_input {
            assert_eq!(
                device.get_device_label(&Scope::Input).unwrap_err(),
                Error::WrongScope
            );
        }
    }
}

#[test]
fn test_get_device_label() {
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
        assert!(!device.get_device_label(&Scope::Input).unwrap().is_empty());
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        assert!(!device.get_device_label(&Scope::Output).unwrap().is_empty());
    }
}

// get_device_name
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_get_device_name_with_invalid_id() {
    let unknown_device = AudioObject::new(kAudioObjectUnknown);
    assert_eq!(
        unknown_device.get_device_name().unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
}

#[test]
fn test_get_device_name() {
    // If we have default input/output devices, then they must have non-empty names.
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
        assert!(!device.get_device_name().unwrap().is_empty());
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        assert!(!device.get_device_name().unwrap().is_empty());
    }
}

// get_device_source_name
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_get_device_source_name_with_invalid_id() {
    let unknown_device = AudioObject::new(kAudioObjectUnknown);
    assert_eq!(
        unknown_device.get_device_source_name(&Scope::Input,).unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
    assert_eq!(
        unknown_device.get_device_source_name(&Scope::Output,).unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
}

#[test]
fn test_get_device_source_name_with_invalid_scope() {
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
        let is_output = device.in_scope(&Scope::Output).unwrap();
        if !is_output {
            assert_eq!(
                device.get_device_source_name(&Scope::Output,).unwrap_err(),
                Error::WrongScope
            );
        }
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        let is_input = device.in_scope(&Scope::Input).unwrap();
        if !is_input {
            assert_eq!(
                device.get_device_source_name(&Scope::Input,).unwrap_err(),
                Error::WrongScope
            );
        }
    }
}

#[test]
fn test_get_device_source_name() {
    // Even we have default input/output devices, we may not get the source
    // names of the devices.
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
        match device.get_device_source_name(&Scope::Input) {
            Ok(name) => {
                assert!(!name.is_empty());
            }
            Err(e) => {
                assert_eq!(
                    e,
                    Error::InvalidParameters(audio_object_utils::Error::UnknownProperty)
                );
            }
        }
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        match device.get_device_source_name(&Scope::Output) {
            Ok(name) => {
                assert!(!name.is_empty());
            }
            Err(e) => {
                assert_eq!(
                    e,
                    Error::InvalidParameters(audio_object_utils::Error::UnknownProperty)
                );
            }
        }
    }
}

// get_device_source
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_get_device_source_with_invalid_id() {
    let unknown_device = AudioObject::new(kAudioObjectUnknown);
    assert_eq!(
        unknown_device.get_device_source(&Scope::Input,).unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
    assert_eq!(
        unknown_device.get_device_source(&Scope::Input,).unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
}

#[test]
fn test_get_device_source_with_invalid_scope() {
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
        let is_output = device.in_scope(&Scope::Output).unwrap();
        if !is_output {
            assert_eq!(
                device.get_device_source(&Scope::Output).unwrap_err(),
                Error::WrongScope
            );
        }
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        let is_input = device.in_scope(&Scope::Input).unwrap();
        if !is_input {
            assert_eq!(
                device.get_device_source(&Scope::Input).unwrap_err(),
                Error::WrongScope
            );
        }
    }
}

#[test]
fn test_get_device_source() {
    // If we can get source from input/output devices, then they must be non-zero values.
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
        if let Ok(source) = device.get_device_source(&Scope::Input) {
            assert_ne!(source, 0);
        }
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        if let Ok(source) = device.get_device_source(&Scope::Output) {
            assert_ne!(source, 0);
        }
    }
}


// in_scope
// --------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_in_scope_with_invalid_id() {
    let unknown_device = AudioObject::new(kAudioObjectUnknown);
    assert_eq!(
        unknown_device.in_scope(&Scope::Input).unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
    assert_eq!(
        unknown_device.in_scope(&Scope::Output).unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
}

#[test]
fn test_in_scope() {
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
        assert!(device.in_scope(&Scope::Input).unwrap());
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        assert!(device.in_scope(&Scope::Output).unwrap());
    }
}

// number_of_streams
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_number_of_streams_with_invalid_id() {
    let unknown_device = AudioObject::new(kAudioObjectUnknown);
    assert_eq!(
        unknown_device.number_of_streams(&Scope::Input,).unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
    assert_eq!(
        unknown_device.number_of_streams(&Scope::Output).unwrap_err(),
        Error::InvalidParameters(audio_object_utils::Error::BadObject)
    );
}

#[test]
fn test_number_of_streams() {
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.number_of_streams(&Scope::Input).unwrap() > 0);
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.number_of_streams(&Scope::Output).unwrap() > 0);
    }
}

// AudioStream
// ============================================================================
// Skip for now ...