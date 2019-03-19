use super::*;

const DEVICE_PROPERTY_ADDRESS: sys::AudioObjectPropertyAddress = sys::AudioObjectPropertyAddress {
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

// Tests for Public Functions
// ============================================================================

// get_property_data
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_get_property_data_with_invalid_id() {
    assert_eq!(
        get_property_data::<sys::AudioObjectID>(
            sys::kAudioObjectUnknown,
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
        )
        .unwrap_err(),
        Error::BadObject
    );
    assert_eq!(
        get_property_data::<sys::AudioObjectID>(
            sys::kAudioObjectUnknown,
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
        )
        .unwrap_err(),
        Error::BadObject
    );
}

#[test]
fn test_get_property_data() {
    assert!(get_property_data::<sys::AudioObjectID>(
        sys::kAudioObjectSystemObject,
        &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
    )
    .is_ok());
    assert!(get_property_data::<sys::AudioObjectID>(
        sys::kAudioObjectSystemObject,
        &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
    )
    .is_ok());
}

// get_property_data_with_ref
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_get_property_data_with_ref_with_invalid_id() {
    let mut id: sys::AudioObjectID = sys::kAudioObjectUnknown;
    assert_eq!(
        get_property_data_with_ref(
            sys::kAudioObjectUnknown,
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
            &mut id,
        )
        .unwrap_err(),
        Error::BadObject
    );
    assert_eq!(
        get_property_data_with_ref(
            sys::kAudioObjectUnknown,
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
            &mut id,
        )
        .unwrap_err(),
        Error::BadObject
    );
}

#[test]
fn test_get_property_data_with_ref() {
    let mut id: sys::AudioObjectID = sys::kAudioObjectUnknown;
    assert!(get_property_data_with_ref(
        sys::kAudioObjectSystemObject,
        &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
        &mut id,
    )
    .is_ok());
    assert!(get_property_data_with_ref(
        sys::kAudioObjectSystemObject,
        &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
        &mut id,
    )
    .is_ok());
}

// get_property_data_size
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_get_property_data_size_with_invalid_id() {
    assert_eq!(
        get_property_data_size(
            sys::kAudioObjectUnknown,
            &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
        )
        .unwrap_err(),
        Error::BadObject
    );
    assert_eq!(
        get_property_data_size(
            sys::kAudioObjectUnknown,
            &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
        )
        .unwrap_err(),
        Error::BadObject
    );
}

#[test]
fn test_get_property_data_size() {
    assert!(get_property_data_size(
        sys::kAudioObjectSystemObject,
        &DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS,
    )
    .is_ok());
    assert!(get_property_data_size(
        sys::kAudioObjectSystemObject,
        &DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS,
    )
    .is_ok());
}

// get_property_array
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_get_property_array_with_invalid_id() {
    assert_eq!(
        get_property_array::<sys::AudioObjectID>(
            sys::kAudioObjectUnknown,
            &DEVICE_PROPERTY_ADDRESS,
        )
        .unwrap_err(),
        Error::BadObject
    );
}

#[test]
fn test_get_property_array() {
    assert!(get_property_array::<sys::AudioObjectID>(
        sys::kAudioObjectSystemObject,
        &DEVICE_PROPERTY_ADDRESS,
    )
    .is_ok());
}

// Tests for Private Functions
// ============================================================================

// non_empty_size
// ------------------------------------
// Skip now ...

// convert_to_result
// ------------------------------------
// Skip now ...

// audio_object_get_property_data
// ------------------------------------
// Skip now ...

// audio_object_get_property_data_size
// ------------------------------------
// Skip now ...

// audio_object_set_property_data
// ------------------------------------
// Skip now ...
