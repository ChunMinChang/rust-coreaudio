use super::*;

// Tests for Public Functions
// ============================================================================

// get_default_device_id
// ------------------------------------
#[test]
fn test_get_default_device_id() {
    // If we have default input/output devices, then they must be valid ids.
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
    }
}

// in_scope
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_in_scope_with_invalid_id() {
    assert_eq!(
        in_scope(&sys::kAudioObjectUnknown, &Scope::Input).unwrap_err(),
        Error::InvalidParameters
    );
    assert_eq!(
        in_scope(&sys::kAudioObjectUnknown, &Scope::Output).unwrap_err(),
        Error::InvalidParameters
    );
}

#[test]
fn test_in_scope() {
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        assert!(in_scope(&id, &Scope::Input).unwrap());
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        assert!(in_scope(&id, &Scope::Output).unwrap());
    }
}

// get_device_ids
// ------------------------------------
#[test]
fn test_get_device_ids() {
    if get_default_device_id(&Scope::Input).is_ok() {
        assert!(!get_device_ids(&Scope::Input).unwrap().is_empty());
    }

    if get_default_device_id(&Scope::Output).is_ok() {
        assert!(!get_device_ids(&Scope::Output).unwrap().is_empty());
    }
}

// get_all_device_ids
// ------------------------------------
#[test]
fn test_get_all_device_ids() {
    if get_default_device_id(&Scope::Input).is_ok() ||
       get_default_device_id(&Scope::Output).is_ok() {
        assert!(!get_all_device_ids().unwrap().is_empty());
    }
}

// get_device_label
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_device_label_with_invalid_id() {
    assert_eq!(
        get_device_label(&sys::kAudioObjectUnknown, &Scope::Input).unwrap_err(),
        Error::InvalidParameters
    );
    assert_eq!(
        get_device_label(&sys::kAudioObjectUnknown, &Scope::Output).unwrap_err(),
        Error::InvalidParameters
    );
}

#[test]
fn test_get_device_label_with_invalid_scope() {
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        let is_output = in_scope(&id, &Scope::Output).unwrap();
        if !is_output {
            assert_eq!(
                get_device_label(&id, &Scope::Output).unwrap_err(),
                Error::NotFound
            );
        }
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        let is_input = in_scope(&id, &Scope::Input).unwrap();
        if !is_input {
            assert_eq!(
                get_device_label(&id, &Scope::Input).unwrap_err(),
                Error::NotFound
            );
        }
    }
}

#[test]
fn test_get_device_label() {
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        assert!(!get_device_label(&id, &Scope::Input).unwrap().is_empty());
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        assert!(!get_device_label(&id, &Scope::Output).unwrap().is_empty());
    }
}

// get_device_name
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_device_name_with_invalid_id() {
    assert_eq!(
        get_device_name(&sys::kAudioObjectUnknown).unwrap_err(),
        Error::InvalidParameters
    );
}

#[test]
fn test_get_device_name() {
    // If we have default input/output devices, then they must have non-empty names.
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        assert!(!get_device_name(&id).unwrap().is_empty());
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        assert!(!get_device_name(&id).unwrap().is_empty());
    }
}

// get_device_source_name
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_device_source_name_with_invalid_id() {
    assert_eq!(
        get_device_source_name(&sys::kAudioObjectUnknown, &Scope::Input,).unwrap_err(),
        Error::InvalidParameters
    );
    assert_eq!(
        get_device_source_name(&sys::kAudioObjectUnknown, &Scope::Output,).unwrap_err(),
        Error::InvalidParameters
    );
}

#[test]
fn test_get_device_source_name_with_invalid_scope() {
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        let is_output = in_scope(&id, &Scope::Output).unwrap();
        if !is_output {
            assert_eq!(
                get_device_source_name(&id, &Scope::Output,).unwrap_err(),
                Error::NotFound
            );
        }
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        let is_input = in_scope(&id, &Scope::Input).unwrap();
        if !is_input {
            assert_eq!(
                get_device_source_name(&id, &Scope::Input,).unwrap_err(),
                Error::NotFound
            );
        }
    }
}

#[test]
fn test_get_device_source_name() {
    // Even we have default input/output devices, we may not get the source
    // names of the devices.
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        match get_device_source_name(&id, &Scope::Input) {
            Ok(name) => {
                assert!(!name.is_empty());
            }
            Err(e) => {
                assert_eq!(e, Error::NotFound);
            }
        }
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        match get_device_source_name(&id, &Scope::Output) {
            Ok(name) => {
                assert!(!name.is_empty());
            }
            Err(e) => {
                assert_eq!(e, Error::NotFound);
            }
        }
    }
}

// set_default_device
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_set_default_device_with_invalid_id() {
    assert_eq!(
        set_default_device(&sys::kAudioObjectUnknown, &Scope::Input).unwrap_err(),
        Error::InvalidParameters
    );
    assert_eq!(
        set_default_device(&sys::kAudioObjectUnknown, &Scope::Output).unwrap_err(),
        Error::InvalidParameters
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
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        assert_eq!(
            set_default_device(&id, &Scope::Input).unwrap_err(),
            Error::InvalidParameters
        );
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        assert_eq!(
            set_default_device(&id, &Scope::Output).unwrap_err(),
            Error::InvalidParameters
        );
    }
}

#[test]
fn test_set_default_device_with_invalid_scope() {
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        let is_output = in_scope(&id, &Scope::Output).unwrap();
        if !is_output {
            assert_eq!(
                set_default_device(&id, &Scope::Output).unwrap_err(),
                Error::InvalidParameters
            );
        }
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        let is_input = in_scope(&id, &Scope::Input).unwrap();
        if !is_input {
            assert_eq!(
                set_default_device(&id, &Scope::Input).unwrap_err(),
                Error::InvalidParameters
            );
        }
    }
}

fn test_change_default_device(scope: &Scope) {
    let devices = get_device_ids(scope).unwrap_or(vec![]);
    if devices.len() < 2 {
        return;
    }

    let current_device = get_default_device_id(scope).unwrap();
    let new_device = devices
        .iter()
        .find(|&device| device != &current_device)
        .unwrap();
    assert!(set_default_device(&new_device, scope).is_ok());
}

#[test]
fn test_set_default_device() {
    test_change_default_device(&Scope::Input);
    test_change_default_device(&Scope::Output);
}

// Tests for Private Functions
// ============================================================================

// get_device_source
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_device_source_with_invalid_id() {
    assert_eq!(
        get_device_source(&sys::kAudioObjectUnknown, &Scope::Input,).unwrap_err(),
        Error::InvalidParameters
    );
    assert_eq!(
        get_device_source(&sys::kAudioObjectUnknown, &Scope::Input,).unwrap_err(),
        Error::InvalidParameters
    );
}

#[test]
fn test_get_device_source_with_invalid_scope() {
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        let is_output = in_scope(&id, &Scope::Output).unwrap();
        if !is_output {
            assert_eq!(
                get_device_source(&id, &Scope::Output).unwrap_err(),
                Error::NotFound
            );
        }
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        let is_input = in_scope(&id, &Scope::Input).unwrap();
        if !is_input {
            assert_eq!(
                get_device_source(&id, &Scope::Input).unwrap_err(),
                Error::NotFound
            );
        }
    }
}

#[test]
fn test_get_device_source() {
    // If we can get source from input/output devices, then they must be non-zero values.
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        match get_device_source(&id, &Scope::Input) {
            Ok(source) => assert_ne!(source, 0),
            _ => {}
        }
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert_ne!(id, sys::kAudioObjectUnknown);
        match get_device_source(&id, &Scope::Output) {
            Ok(source) => assert_ne!(source, 0),
            _ => {}
        }
    }
}

// number_of_streams
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_number_of_streams_with_invalid_id() {
    assert_eq!(
        number_of_streams(&sys::kAudioObjectUnknown, &Scope::Input,).unwrap_err(),
        Error::InvalidParameters
    );
    assert_eq!(
        number_of_streams(&sys::kAudioObjectUnknown, &Scope::Output).unwrap_err(),
        Error::InvalidParameters
    );
}

#[test]
fn test_number_of_streams() {
    if let Ok(id) = get_default_device_id(&Scope::Input) {
        assert!(number_of_streams(&id, &Scope::Input).unwrap() > 0);
    }

    if let Ok(id) = get_default_device_id(&Scope::Output) {
        assert!(number_of_streams(&id, &Scope::Output).unwrap() > 0);
    }
}

// btye_array_to_string
// ------------------------------------
#[test]
fn test_btye_array_to_string() {
    let c_str = CStr::from_bytes_with_nul(b"hello~!@#$%^&*()_-=+\0").unwrap();
    let v_u8 = c_str.to_bytes().to_vec();
    let v_i8 = v_u8.iter().map(|&e| e as i8).collect();
    let result = btye_array_to_string(v_i8).unwrap();
    let expected = c_str.to_str().unwrap().to_string();
    assert_eq!(expected, result);
}

// get_property_data
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_property_data_with_invalid_id() {
    assert_eq!(
        get_property_data::<sys::AudioObjectID>(
            &sys::kAudioObjectUnknown,
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
        ).unwrap_err(),
        Error::InvalidParameters
    );
    assert_eq!(
        get_property_data::<sys::AudioObjectID>(
            &sys::kAudioObjectUnknown,
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
        ).unwrap_err(),
        Error::InvalidParameters
    );
}

#[test]
fn test_get_property_data() {
    assert!(
        get_property_data::<sys::AudioObjectID>(
            &sys::kAudioObjectSystemObject,
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
        ).is_ok()
    );
    assert!(
        get_property_data::<sys::AudioObjectID>(
            &sys::kAudioObjectSystemObject,
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
        ).is_ok()
    );
}

// get_property_data_with_ptr
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_property_data_with_ptr_with_invalid_id() {
    let mut id: sys::AudioObjectID = sys::kAudioObjectUnknown;
    assert_eq!(
        get_property_data_with_ptr(
            &sys::kAudioObjectUnknown,
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
            &mut id,
        ).unwrap_err(),
        Error::InvalidParameters
    );
    assert_eq!(
        get_property_data_with_ptr(
            &sys::kAudioObjectUnknown,
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
            &mut id,
        ).unwrap_err(),
        Error::InvalidParameters
    );
}

#[test]
fn test_get_property_data_with_ptr() {
    let mut id: sys::AudioObjectID = sys::kAudioObjectUnknown;
    assert!(
        get_property_data_with_ptr(
            &sys::kAudioObjectSystemObject,
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
            &mut id,
        ).is_ok()
    );
    assert!(
        get_property_data_with_ptr(
            &sys::kAudioObjectSystemObject,
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
            &mut id,
        ).is_ok()
    );
}

// get_property_data_size
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_property_data_size_with_invalid_id() {
    assert_eq!(
        get_property_data_size(
            &sys::kAudioObjectUnknown,
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
        ).unwrap_err(),
        Error::InvalidParameters
    );
    assert_eq!(
        get_property_data_size(
            &sys::kAudioObjectUnknown,
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
        ).unwrap_err(),
        Error::InvalidParameters
    );
}

#[test]
fn test_get_property_data_size() {
    assert!(
        get_property_data_size(
            &sys::kAudioObjectSystemObject,
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
        ).is_ok()
    );
    assert!(
        get_property_data_size(
            &sys::kAudioObjectSystemObject,
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
        ).is_ok()
    );
}

// get_property_array
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_property_array_with_invalid_id() {
    assert_eq!(
        get_property_array::<sys::AudioObjectID>(
            &sys::kAudioObjectUnknown,
            &DEVICE_PROPERTY_ADDRESS,
        ).unwrap_err(),
        Error::InvalidParameters
    );
    assert_eq!(
        get_property_array::<sys::AudioObjectID>(
            &sys::kAudioObjectUnknown,
            &DEVICE_PROPERTY_ADDRESS,
        ).unwrap_err(),
        Error::InvalidParameters
    );
}

#[test]
fn test_get_property_array() {
    assert!(
        get_property_array::<sys::AudioObjectID>(
            &sys::kAudioObjectSystemObject,
            &DEVICE_PROPERTY_ADDRESS,
        ).is_ok()
    );
    assert!(
        get_property_array::<sys::AudioObjectID>(
            &sys::kAudioObjectSystemObject,
            &DEVICE_PROPERTY_ADDRESS,
        ).is_ok()
    );
}
