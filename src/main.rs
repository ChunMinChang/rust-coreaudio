extern crate rust_coreaudio;
use rust_coreaudio::utils; // Refer to `utils` module

fn show_result() {
    for id in &utils::get_all_device_ids().unwrap() {
        show_device_info(id);
    }
}

fn show_device_info(id: &u32) {
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

fn get_device_info(id: &u32, scope: &utils::Scope) -> String {
    let default_id = utils::get_default_device_id(scope).unwrap();
    let mut info = String::from(if id == &default_id { "(default " } else { "(" });
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

fn main() {
    show_result();
}