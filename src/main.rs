extern crate rust_coreaudio;
use rust_coreaudio::utils;
use rust_coreaudio::stream;

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
    let default_id = utils::get_default_device_id(scope).unwrap();
    let mut info = String::from(if id == default_id { "(default " } else { "(" });
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

    let current_device = utils::get_default_device_id(scope).unwrap();
    let new_device = devices
        .into_iter()
        .find(|&device| device != current_device)
        .unwrap();
    assert!(utils::set_default_device(new_device, scope).is_ok());
}

fn play_sound() {
    use stream::{Buffer, CallbackArgs, Format, Stream};
    use std::f64::consts::PI;

    struct Iter {
        value: f64,
    }
    impl Iterator for Iter {
        type Item = f64;
        fn next(&mut self) -> Option<f64> {
            self.value += 440.0 / 44_100.0;
            Some(self.value)
        }
    }

    // 440hz sine wave generator.
    let mut samples = Iter { value: 0.0 }.map(|phase| (phase * PI * 2.0).sin() as f32 * 0.15);

    type Args = CallbackArgs<Buffer<f32>>;
    let stm = Stream::new(2, Format::F32LE, 44100.0, move |args| {
        let Args { mut data, frames } = args;
        for i in 0..frames {
            let sample = samples.next().unwrap();
            for channel in data.channels_mut() {
                channel[i] = sample;
            }
        }
    }).unwrap();

    stm.start().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(3000));

    stm.stop().unwrap();
}

fn main() {
    play_sound();
    // show_result();
    // change_default_devices();
}
