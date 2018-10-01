extern crate rust_coreaudio;
use rust_coreaudio::utils;

fn show_result() {
    for device in utils::get_all_devices().unwrap() {
        show_device_info(&device);
    }
}

fn show_device_info(device: &utils::AudioObject) {
    print!("Device {:?}: ", device);

    let scopes = vec![
        utils::Scope::Input,
        utils::Scope::Output
    ];

    let mut info: String = String::from("");
    for scope in &scopes {
        if !device.in_scope(scope).unwrap() {
            continue;
        }
        if !info.is_empty() {
            info += ", ";
        }
        info += get_device_info(device, scope).as_ref();
    }
    println!("{}", info);
}

fn get_device_info(device: &utils::AudioObject, scope: &utils::Scope) -> String {
    let default_device = utils::get_default_device(scope).unwrap();
    let mut info = String::from(
        if device == &default_device {
            "(default "
        } else {
            "("
        }
    );
    info += (to_string(scope) + ") ").as_ref();
    info += device.get_device_label(scope).unwrap().as_ref();
    info += (" <channels: ".to_owned() + device.get_channel_count(scope).unwrap().to_string().as_str() + ">").as_str();
    info
}

fn change_default_devices() {
    change_default_device(&utils::Scope::Input);
    change_default_device(&utils::Scope::Output);
}

fn change_default_device(scope: &utils::Scope) {
    let devices = utils::get_devices(scope).unwrap_or_default();
    if devices.len() < 2 {
        return;
    }

    let current_device = utils::get_default_device(scope).unwrap();
    let new_device = devices
        .into_iter()
        .find(|ref device| device != &&current_device)
        .unwrap();
    assert!(utils::set_default_device(&new_device, scope).is_ok());

    println!("{} device is changed from {:?} to {:?}!",
             to_string(scope),
             current_device.get_device_label(scope).unwrap(),
             new_device.get_device_label(scope).unwrap());
}

fn to_string(scope: &utils::Scope) -> String {
    if scope == &utils::Scope::Input {
        "Input".to_string()
    } else {
        "Output".to_string()
    }
}

fn main() {
    show_result();
    change_default_devices();
}
