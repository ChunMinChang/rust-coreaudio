extern crate rust_coreaudio;
use rust_coreaudio::utils;

struct DeviceInfo {
    id: u32,
    label: String,
    uid: String,
    manufacturer: String,
    channels: u32,
    rate: f64,
    rate_range: (f64, f64),
    device_latency: u32,
    stream_latency: u32,
    buf_frame_size_range: (f64, f64),
}

impl DeviceInfo {
    fn new(
        id: u32,
        label: String,
        uid: String,
        manufacturer: String,
        channels: u32,
        rate: f64,
        rate_range: (f64, f64),
        device_latency: u32,
        stream_latency: u32,
        buf_frame_size_range: (f64, f64),
    ) -> Self {
        DeviceInfo {
            id,
            label,
            uid,
            manufacturer,
            channels,
            rate,
            rate_range,
            device_latency,
            stream_latency,
            buf_frame_size_range,
        }
    }
}

fn print_devices_info() {
    let scopes = vec![utils::Scope::Input, utils::Scope::Output];

    for scope in &scopes {
        print_devices_info_in_scope(scope);
    }
}

fn print_devices_info_in_scope(scope: &utils::Scope) {
    print_demarcation(scope);
    let devices = utils::get_all_devices().unwrap();
    for device in devices {
        if !device.in_scope(scope).unwrap() {
            continue;
        }

        let info = get_device_info(&device, scope).unwrap();
        print_device_info(&info);
    }
}

fn get_device_info(device: &utils::AudioObject, scope: &utils::Scope) -> Option<DeviceInfo> {
    if !device.in_scope(scope).unwrap() {
        return None;
    }

    Some(DeviceInfo::new(
        utils::GetObjectId::get_id(device),
        device.get_label(scope).unwrap(),
        device.get_uid().unwrap(),
        device.get_manufacturer().unwrap(),
        device.get_channel_count(scope).unwrap(),
        device.get_rate(scope).unwrap(),
        device.get_rate_range(scope).unwrap(),
        device.get_device_latency(scope).unwrap(),
        device.get_stream_latency(scope).unwrap(),
        device.get_buffer_frame_size_range(scope).unwrap(),
    ))
}

fn print_device_info(info: &DeviceInfo) {
    println!("{}: {}", info.id, info.label);
    println!("\tuid: {}", info.uid);
    println!("\tmanufacturer: {}", info.manufacturer);
    println!("\tchannels: {}", info.channels);
    println!("\trate: {}", info.rate);
    println!(
        "\trate range: {} - {}",
        info.rate_range.0, info.rate_range.1
    );
    println!("\tdevice latency: {}", info.device_latency);
    println!("\tstream latency: {}", info.stream_latency);
    println!(
        "\tbuffer frame size range: {} - {}",
        info.buf_frame_size_range.0, info.buf_frame_size_range.1
    );
}

fn print_demarcation(scope: &utils::Scope) {
    println!("{}\n----------", to_string(scope));
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

    println!(
        "{} device is changed from {:?} to {:?}!",
        to_string(scope),
        current_device.get_label(scope).unwrap(),
        new_device.get_label(scope).unwrap()
    );
}

fn to_string(scope: &utils::Scope) -> String {
    if scope == &utils::Scope::Input {
        "Input".to_string()
    } else {
        "Output".to_string()
    }
}

fn main() {
    print_devices_info();
    change_default_devices();
}
