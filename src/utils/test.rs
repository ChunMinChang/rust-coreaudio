use super::*;

// Tests for Public Functions
// ============================================================================

// get_default_device
// ------------------------------------
#[test]
fn test_get_default_device() {
    // If we have default input/output devices, then they must be valid ids.
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
    }
}

// get_device_ids
// ------------------------------------
#[test]
fn test_get_devices() {
    if get_default_device(&Scope::Input).is_ok() {
        assert!(!get_devices(&Scope::Input).unwrap().is_empty());
    }

    if get_default_device(&Scope::Output).is_ok() {
        assert!(!get_devices(&Scope::Output).unwrap().is_empty());
    }
}

// get_all_device_ids
// ------------------------------------
#[test]
fn test_get_all_devices() {
    if get_default_device(&Scope::Input).is_ok() ||
       get_default_device(&Scope::Output).is_ok() {
        assert!(!get_all_devices().unwrap().is_empty());
    }
}


// set_default_device
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_set_default_device_with_invalid_id() {
    let unknown_device = AudioObject::new(kAudioObjectUnknown);
    assert_eq!(
        set_default_device(&unknown_device, &Scope::Input).unwrap_err(),
        Error::InvalidParameters(audio_object::Error::BadObject)
    );
    assert_eq!(
        set_default_device(&unknown_device, &Scope::Output).unwrap_err(),
        Error::InvalidParameters(audio_object::Error::BadObject)
    );
}

// NOTICE: test_set_default_device_with_same_device() will be interfered with
// test_set_default_device() when running test in parallel threads.
//
// Suppose we have two threads, 1 and 2:
// - test_set_default_device_with_same_device() runs on thread 1
// - test_set_default_device() runs on thread 2
// After starting running tests, CPU will switch to run thread 1 and thread 2.
// If CPU stops running thread 1 upon getting the result from
// get_default_device_id(...) and switch to finish all the tasks on thread 2,
// then the defult device will be changed. And then if CPU switch back to run
// ib thread 1, the default device id is different now from what thread 1 get
// earlier. It cause a fail to call set_default_device(...).unwrap_err().
//
// To avoid this race, we can use the following command to run all the tests:
// $ cargo test -- --test-threads=1
// or
// $ cargo test -- set_default_device --test-threads 1
#[test]
fn test_set_default_device_with_same_device() {
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
        assert_eq!(
            set_default_device(&device, &Scope::Input).unwrap_err(),
            Error::SetSameDevice
        );
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        assert_eq!(
            set_default_device(&device, &Scope::Output).unwrap_err(),
            Error::SetSameDevice
        );
    }
}

#[test]
fn test_set_default_device_with_invalid_scope() {
    if let Ok(device) = get_default_device(&Scope::Input) {
        assert!(device.is_valid());
        let is_output = device.in_scope(&Scope::Output).unwrap();
        if !is_output {
            assert_eq!(
                set_default_device(&device, &Scope::Output).unwrap_err(),
                Error::WrongScope
            );
        }
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        let is_input = device.in_scope(&Scope::Input).unwrap();
        if !is_input {
            assert_eq!(
                set_default_device(&device, &Scope::Input).unwrap_err(),
                Error::WrongScope
            );
        }
    }
}

fn test_change_default_device(scope: &Scope) {
    let devices = get_devices(scope).unwrap_or_default();
    if devices.len() < 2 {
        return;
    }

    let current_device = get_default_device(scope).unwrap();
    let new_device = devices
        .into_iter()
        .find(|ref device| device != &&current_device)
        .unwrap();
    assert!(set_default_device(&new_device, scope).is_ok());
}

#[test]
#[ignore]
fn test_set_default_device() {
    test_change_default_device(&Scope::Input);
    test_change_default_device(&Scope::Output);
}


// AudioObject
// ------------------------------------
// get_device_label
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_get_device_label_with_invalid_id() {
    let unknown_device = AudioObject::new(kAudioObjectUnknown);
    assert_eq!(
        unknown_device.get_device_label(&Scope::Input).unwrap_err(),
        Error::InvalidParameters(audio_object::Error::BadObject)
    );
    assert_eq!(
        unknown_device.get_device_label(&Scope::Output).unwrap_err(),
        Error::InvalidParameters(audio_object::Error::BadObject)
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
        Error::InvalidParameters(audio_object::Error::BadObject)
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
        Error::InvalidParameters(audio_object::Error::BadObject)
    );
    assert_eq!(
        unknown_device.get_device_source_name(&Scope::Output,).unwrap_err(),
        Error::InvalidParameters(audio_object::Error::BadObject)
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
                    Error::InvalidParameters(audio_object::Error::UnknownProperty)
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
                    Error::InvalidParameters(audio_object::Error::UnknownProperty)
                );
            }
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
        Error::InvalidParameters(audio_object::Error::BadObject)
    );
    assert_eq!(
        unknown_device.in_scope(&Scope::Output).unwrap_err(),
        Error::InvalidParameters(audio_object::Error::BadObject)
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

// get_device_source
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_get_device_source_with_invalid_id() {
    let unknown_device = AudioObject::new(kAudioObjectUnknown);
    assert_eq!(
        unknown_device.get_device_source(&Scope::Input,).unwrap_err(),
        Error::InvalidParameters(audio_object::Error::BadObject)
    );
    assert_eq!(
        unknown_device.get_device_source(&Scope::Input,).unwrap_err(),
        Error::InvalidParameters(audio_object::Error::BadObject)
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

// number_of_streams
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_number_of_streams_with_invalid_id() {
    let unknown_device = AudioObject::new(kAudioObjectUnknown);
    assert_eq!(
        unknown_device.number_of_streams(&Scope::Input,).unwrap_err(),
        Error::InvalidParameters(audio_object::Error::BadObject)
    );
    assert_eq!(
        unknown_device.number_of_streams(&Scope::Output).unwrap_err(),
        Error::InvalidParameters(audio_object::Error::BadObject)
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

// Tests for Private Functions
// ============================================================================
// AudioStream
// ------------------------------------
// Skip for now ...

// AudioSystemObject
// ------------------------------------
// Skip for now ...