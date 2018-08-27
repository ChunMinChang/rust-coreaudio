use super::*;

// Tests for Public Functions
// ============================================================================

// struct AudioUnit
// ------------------------------------
// Skip now ...

// Tests for Private Functions
// ============================================================================

// create_unit
// ------------------------------------
#[test]
fn test_create_unit() {
    let output = create_unit().unwrap();
    assert!(!output.is_null());
}

// get_property_info
// ------------------------------------
// Skip now ...

// get_property
// ------------------------------------
// Skip now ...

// set_property
// ------------------------------------
// Skip now ...

// find_next_component
// ------------------------------------
#[test]
fn test_find_next_component_with_invalid_description() {
    let desc = sys::AudioComponentDescription {
        componentType: sys::kAudioUnitType_Effect,
        componentSubType: sys::kAudioUnitSubType_DefaultOutput,
        componentManufacturer: sys::kAudioUnitManufacturer_Apple,
        componentFlags: 0,
        componentFlagsMask: 0,
    };
    assert_eq!(
        find_next_component(ptr::null_mut(), &desc).unwrap_err(),
        Error::NoComponentFound
    );
}

// TODO: it may fail when there is no output device.
#[test]
fn test_find_next_component() {
    let desc = sys::AudioComponentDescription {
        componentType: sys::kAudioUnitType_Output,
        componentSubType: sys::kAudioUnitSubType_DefaultOutput,
        componentManufacturer: sys::kAudioUnitManufacturer_Apple,
        componentFlags: 0,
        componentFlagsMask: 0,
    };
    let component = find_next_component(ptr::null_mut(), &desc).unwrap();
    assert!(!component.is_null());
}

// get_new_instance
// ------------------------------------
#[test]
fn test_get_new_instance_with_invalid_component() {
    assert_eq!(
        get_new_instance(ptr::null_mut()).unwrap_err(),
        Error::InvalidComponentID
    );
}

#[test]
fn test_get_new_instance() {
    let desc = sys::AudioComponentDescription {
        componentType: sys::kAudioUnitType_Output,
        componentSubType: sys::kAudioUnitSubType_DefaultOutput,
        componentManufacturer: sys::kAudioUnitManufacturer_Apple,
        componentFlags: 0,
        componentFlagsMask: 0,
    };
    let component = find_next_component(ptr::null_mut(), &desc).unwrap();
    let instance = get_new_instance(component).unwrap();
    assert!(!instance.is_null());
}

// convert_to_result
// ------------------------------------
// Skip now ...

// status_to_error
// ------------------------------------
// Skip now ...

// to_bindgen_type
// ------------------------------------
// Skip now ...

// audio_unit_get_property_info
// ------------------------------------
// Skip now ...

// audio_unit_get_property
// ------------------------------------
// Skip now ...

// audio_unit_set_property
// ------------------------------------
// Skip now ...

// audio_component_find_next
// ------------------------------------
// Skip now ...

// audio_component_instance_new
// ------------------------------------
// Skip now ...
