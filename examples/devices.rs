extern crate rust_coreaudio;
use rust_coreaudio::utils;

fn show_result() {
    for id in utils::get_all_device_ids().unwrap() {
        show_device_info(id);
    }
}

fn show_device_info(id: u32) {
    print!("Device {}: ", id);
    let mut info: String = String::from("");
    let scopes = vec![utils::Scope::Input, utils::Scope::Output];
    for scope in &scopes {
        if !utils::in_scope(id, scope).unwrap() {
            continue;
        }
        if !info.is_empty() {
            info += ", ";
        }
        info += get_device_info(id, scope).as_ref();
    }
    println!("{}", info);
}

fn get_device_info(id: u32, scope: &utils::Scope) -> String {
    let default_device = utils::get_default_device(scope).unwrap();
    let mut info = String::from(if id == default_device.into() { "(default " } else { "(" });
    info += (to_string(scope) + ") ").as_ref();
    info += utils::get_device_label(id, scope).unwrap().as_ref();
    info
}

fn to_string(scope: &utils::Scope) -> String {
    if scope == &utils::Scope::Input {
        "Input".to_string()
    } else {
        "Output".to_string()
    }
}

fn change_default_devices() {
    change_default_device(&utils::Scope::Input);
    change_default_device(&utils::Scope::Output);
}

fn change_default_device(scope: &utils::Scope) {
    let devices = utils::get_device_ids(scope).unwrap_or_default();
    if devices.len() < 2 {
        return;
    }

    let current_device = utils::get_default_device(scope).unwrap();
    let new_device = devices
        .into_iter()
        .find(|&device| device != current_device.clone().into()) // TODO: Ugly!
        .unwrap();
    assert!(utils::set_default_device(new_device, scope).is_ok());
}

fn main() {
    show_result();
    change_default_devices();
}
