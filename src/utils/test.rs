use super::*;

// Tests for Public Functions
// ============================================================================

// get_default_device_id
// ------------------------------------
#[test]
fn test_get_default_device_id() {
    // If we have default input/output devices, then they must be valid ids.
    match get_default_device_id(&Scope::Input) {
        Ok(id) => { assert_ne!(id, sys::kAudioObjectUnknown); },
        _ => {},
    }

    match get_default_device_id(&Scope::Output) {
        Ok(id) => { assert_ne!(id, sys::kAudioObjectUnknown); },
        _ => {},
    }
}

// in_scope
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_in_scope_with_invalid_id() {
    assert_eq!(in_scope(&sys::kAudioObjectUnknown, &Scope::Input).unwrap_err(),
                        Error::InvalidParameters);
    assert_eq!(in_scope(&sys::kAudioObjectUnknown, &Scope::Output).unwrap_err(),
                        Error::InvalidParameters);
}

#[test]
fn test_in_scope() {
    match get_default_device_id(&Scope::Input) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            assert!(in_scope(&id, &Scope::Input).unwrap());
        },
        _ => {},
    }

    match get_default_device_id(&Scope::Output) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            assert!(in_scope(&id, &Scope::Output).unwrap());
        },
        _ => {},
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
    assert_eq!(get_device_label(
                   &sys::kAudioObjectUnknown,
                   &Scope::Input
               ).unwrap_err(), Error::InvalidParameters);
    assert_eq!(get_device_label(
                   &sys::kAudioObjectUnknown,
                   &Scope::Output
               ).unwrap_err(), Error::InvalidParameters);
}

#[test]
fn test_get_device_label_with_invalid_scope() {
    match get_default_device_id(&Scope::Input) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            let is_output = in_scope(&id, &Scope::Output).unwrap();
            if !is_output {
                assert_eq!(get_device_label(&id, &Scope::Output).unwrap_err(),
                           Error::NotFound);
            }
        },
        _ => {},
    }

    match get_default_device_id(&Scope::Output) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            let is_input = in_scope(&id, &Scope::Input).unwrap();
            if !is_input {
                assert_eq!(get_device_label(&id, &Scope::Input).unwrap_err(),
                           Error::NotFound);
            }
        },
        _ => {},
    }
}

#[test]
fn test_get_device_label() {
    match get_default_device_id(&Scope::Input) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            assert!(!get_device_label(&id, &Scope::Input).unwrap().is_empty());
        },
        _ => {},
    }

    match get_default_device_id(&Scope::Output) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            assert!(!get_device_label(&id, &Scope::Output).unwrap().is_empty());
        },
        _ => {},
    }
}

// get_device_name
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_device_name_with_invalid_id() {
    assert_eq!(get_device_name(&sys::kAudioObjectUnknown).unwrap_err(),
               Error::InvalidParameters);
}

#[test]
fn test_get_device_name() {
    // If we have default input/output devices, then they must have non-empty names.
    match get_default_device_id(&Scope::Input) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            assert!(!get_device_name(&id).unwrap().is_empty());
        },
        _ => {},
    }

    match get_default_device_id(&Scope::Output) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            assert!(!get_device_name(&id).unwrap().is_empty());
        },
        _ => {},
    }
}

// get_device_source_name
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_device_source_name_with_invalid_id() {
    assert_eq!(get_device_source_name(
                   &sys::kAudioObjectUnknown,
                   &Scope::Input,
               ).unwrap_err(), Error::InvalidParameters);
    assert_eq!(get_device_source_name(
                   &sys::kAudioObjectUnknown,
                   &Scope::Output,
               ).unwrap_err(), Error::InvalidParameters);
}

#[test]
fn test_get_device_source_name_with_invalid_scope() {
    match get_default_device_id(&Scope::Input) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            let is_output = in_scope(&id, &Scope::Output).unwrap();
            if !is_output {
                assert_eq!(get_device_source_name(
                               &id,
                               &Scope::Output,
                           ).unwrap_err(), Error::NotFound);
            }
        },
        _ => {},
    }

    match get_default_device_id(&Scope::Output) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            let is_input = in_scope(&id, &Scope::Input).unwrap();
            if !is_input {
                assert_eq!(get_device_source_name(
                               &id,
                               &Scope::Input,
                           ).unwrap_err(), Error::NotFound);
            }
        },
        _ => {},
    }
}

#[test]
fn test_get_device_source_name() {
    match get_default_device_id(&Scope::Input) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            match get_device_source_name(&id, &Scope::Input) {
                Ok(name) => { assert!(!name.is_empty()); }
                Err(e) => { assert_eq!(e, Error::NotFound); }
            }
        },
        _ => {},
    }

    match get_default_device_id(&Scope::Output) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            match get_device_source_name(&id, &Scope::Output) {
                Ok(name) => { assert!(!name.is_empty()); }
                Err(e) => { assert_eq!(e, Error::NotFound); }
            }
        },
        _ => {},
    }
}

// Tests for Private Functions
// ============================================================================

// get_device_source
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_device_source_with_invalid_id() {
    assert_eq!(get_device_source(
                  &sys::kAudioObjectUnknown,
                  &Scope::Input,
              ).unwrap_err(), Error::InvalidParameters);
    assert_eq!(get_device_source(
                  &sys::kAudioObjectUnknown,
                  &Scope::Input,
              ).unwrap_err(), Error::InvalidParameters);
}

#[test]
fn test_get_device_source_with_invalid_scope() {
    match get_default_device_id(&Scope::Input) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            let is_output = in_scope(&id, &Scope::Output).unwrap();
            if !is_output {
                assert_eq!(get_device_source(&id, &Scope::Output).unwrap_err(),
                           Error::NotFound);
            }
        },
        _ => {},
    }

    match get_default_device_id(&Scope::Output) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            let is_input = in_scope(&id, &Scope::Input).unwrap();
            if !is_input {
                assert_eq!(get_device_source(&id, &Scope::Input).unwrap_err(),
                           Error::NotFound);
            }
        },
        _ => {},
    }
}

#[test]
fn test_get_device_source() {
    // If we can get source from input/output devices, then they must be non-zero values.
    match get_default_device_id(&Scope::Input) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            match get_device_source(&id, &Scope::Input) {
                Ok(source) => assert_ne!(source, 0),
                _ => {},
            }
        },
        _ => {},
    }

    match get_default_device_id(&Scope::Output) {
        Ok(id) => {
            assert_ne!(id, sys::kAudioObjectUnknown);
            match get_device_source(&id, &Scope::Output) {
                Ok(source) => assert_ne!(source, 0),
                _ => {},
            }
        },
        _ => {},
    }
}

// number_of_streams
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_number_of_streams_with_invalid_id() {
    assert_eq!(number_of_streams(
                   &sys::kAudioObjectUnknown,
                   &Scope::Input,
               ).unwrap_err(), Error::InvalidParameters);
    assert_eq!(number_of_streams(
                   &sys::kAudioObjectUnknown,
                   &Scope::Output
               ).unwrap_err(), Error::InvalidParameters);
}

#[test]
fn test_number_of_streams() {
    match get_default_device_id(&Scope::Input) {
        Ok(id) => {
            assert!(number_of_streams(&id, &Scope::Input).unwrap() > 0);
        },
        _ => {},
    }

    match get_default_device_id(&Scope::Output) {
        Ok(id) => {
            assert!(number_of_streams(&id, &Scope::Output).unwrap() > 0);
        },
        _ => {},
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
    assert_eq!(get_property_data::<sys::AudioObjectID>(
                   &sys::kAudioObjectUnknown,
                   &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
               ).unwrap_err(), Error::InvalidParameters);
    assert_eq!(get_property_data::<sys::AudioObjectID>(
                   &sys::kAudioObjectUnknown,
                   &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
               ).unwrap_err(), Error::InvalidParameters);
}

#[test]
fn test_get_property_data() {
    assert!(get_property_data::<sys::AudioObjectID>(
                &sys::kAudioObjectSystemObject,
                &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
            ).is_ok());
    assert!(get_property_data::<sys::AudioObjectID>(
                &sys::kAudioObjectSystemObject,
                &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
            ).is_ok());
}

// get_property_data_with_ptr
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_property_data_with_ptr_with_invalid_id() {
    let mut id: sys::AudioObjectID = sys::kAudioObjectUnknown;
    assert_eq!(get_property_data_with_ptr(
                   &sys::kAudioObjectUnknown,
                   &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
                   &mut id,
               ).unwrap_err(), Error::InvalidParameters);
    assert_eq!(get_property_data_with_ptr(
                   &sys::kAudioObjectUnknown,
                   &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
                   &mut id,
               ).unwrap_err(), Error::InvalidParameters);
}

#[test]
fn test_get_property_data_with_ptr() {
    let mut id: sys::AudioObjectID = sys::kAudioObjectUnknown;
    assert!(get_property_data_with_ptr(
                &sys::kAudioObjectSystemObject,
                &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
                &mut id,
            ).is_ok());
    assert!(get_property_data_with_ptr(
                &sys::kAudioObjectSystemObject,
                &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
                &mut id,
            ).is_ok());
}

// get_property_data_size
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_property_data_size_with_invalid_id() {
    assert_eq!(get_property_data_size(
                   &sys::kAudioObjectUnknown,
                   &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
               ).unwrap_err(), Error::InvalidParameters);
    assert_eq!(get_property_data_size(
                   &sys::kAudioObjectUnknown,
                   &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
               ).unwrap_err(), Error::InvalidParameters);
}

#[test]
fn test_get_property_data_size() {
    assert!(get_property_data_size(
                &sys::kAudioObjectSystemObject,
                &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
            ).is_ok());
    assert!(get_property_data_size(
                &sys::kAudioObjectSystemObject,
                &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
            ).is_ok());
}

// get_property_array
// ------------------------------------
#[test]
#[should_panic(expected = "Invalid")]
fn test_get_property_array_with_invalid_id() {
    assert_eq!(get_property_array::<sys::AudioObjectID>(
                   &sys::kAudioObjectUnknown,
                   &DEVICE_PROPERTY_ADDRESS,
               ).unwrap_err(), Error::InvalidParameters);
    assert_eq!(get_property_array::<sys::AudioObjectID>(
                   &sys::kAudioObjectUnknown,
                   &DEVICE_PROPERTY_ADDRESS,
               ).unwrap_err(), Error::InvalidParameters);
}

#[test]
fn test_get_property_array() {
    assert!(get_property_array::<sys::AudioObjectID>(
                &sys::kAudioObjectSystemObject,
                &DEVICE_PROPERTY_ADDRESS,
            ).is_ok());
    assert!(get_property_array::<sys::AudioObjectID>(
                &sys::kAudioObjectSystemObject,
                &DEVICE_PROPERTY_ADDRESS,
            ).is_ok());
}