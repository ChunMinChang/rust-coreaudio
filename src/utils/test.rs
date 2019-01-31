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
    if get_default_device(&Scope::Input).is_ok() || get_default_device(&Scope::Output).is_ok() {
        assert!(!get_all_devices().unwrap().is_empty());
    }
}

// set_default_device
// ------------------------------------
#[test]
// #[should_panic(expected = "Bad")]
fn test_set_default_device_with_invalid_id() {
    let unknown_device = audio_objects::AudioObject::default();
    // TODO: Check error type.
    assert!(!set_default_device(&unknown_device, &Scope::Input).is_ok());
    assert!(!set_default_device(&unknown_device, &Scope::Output).is_ok());
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
            Error::AudioObjects(audio_objects::Error::SetSameDevice)
        );
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        assert_eq!(
            set_default_device(&device, &Scope::Output).unwrap_err(),
            Error::AudioObjects(audio_objects::Error::SetSameDevice)
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
                Error::AudioObjects(audio_objects::Error::WrongScope)
            );
        }
    }

    if let Ok(device) = get_default_device(&Scope::Output) {
        assert!(device.is_valid());
        let is_input = device.in_scope(&Scope::Input).unwrap();
        if !is_input {
            assert_eq!(
                set_default_device(&device, &Scope::Input).unwrap_err(),
                Error::AudioObjects(audio_objects::Error::WrongScope)
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
