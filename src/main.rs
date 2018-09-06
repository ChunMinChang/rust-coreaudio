// Introduce `lazy_static` to use a mutable global variable.
#[macro_use]
extern crate lazy_static;
// It needs to wrap a `Mutex` around the global variable to make it mutable.
use std::sync::Mutex;

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

struct SynthesizedData(f64);
impl From<SynthesizedData> for f32 {
    fn from(d: SynthesizedData) -> f32 {
        d.0 as f32
    }
}
impl From<SynthesizedData> for i16 {
    fn from(d: SynthesizedData) -> i16 {
        (d.0 * 32767.0) as i16
    }
}

struct Synthesizer {
    channels: u32,
    rate: f64,
    volume: f64,
    phase: Vec<f64>,
}
impl Synthesizer {
    fn new(channels: u32, rate: f64, volume: f64) -> Self {
        let phase = vec![0.0; channels as usize];
        Synthesizer {
            channels,
            rate,
            volume,
            phase,
        }
    }

    fn run<T> (&mut self, buffers: &mut [&mut [T]])
    where T: std::convert::From<SynthesizedData> {
        assert_eq!(self.channels, buffers.len() as u32);
        // buffers.len() is equal to channels!
        for (channel, buffer) in buffers.iter_mut().enumerate() {
            // buffer.len() is equal to frames!
            for data in buffer.iter_mut() {
                let channel_data = SynthesizedData(self.phase[channel].sin() * self.volume);
                *data = channel_data.into();
                self.phase[channel as usize] += self.get_increment(channel as u32);
            }
        }
    }

    fn get_increment(&self, channel: u32) -> f64 {
        use std::f64::consts::PI;
        let freq = 220.0 * ((channel + 1) as f64);
        2.0 * PI * freq / self.rate
    }
}

const CHANNELS: u32 = 2;
const RATE: f64 = 44_100.0;
const VOLUME: f64 = 0.5;
lazy_static! {
    static ref SYNTHESIZER: Mutex<Synthesizer> = Mutex::new(Synthesizer::new(CHANNELS, RATE, VOLUME));
}

fn fill_buffer_float(buffers: &mut [&mut [f32]]) {
    SYNTHESIZER.lock().unwrap().run(buffers);
}

fn play_sound_float() {
    use stream::{Format, Stream};

    println!("Play `float` stream");
    let mut stm = Stream::new(CHANNELS, Format::F32LE, RATE, fill_buffer_float).unwrap();
    stm.init().unwrap();
    stm.start().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    stm.stop().unwrap();
}

fn fill_buffer_short(buffers: &mut[&mut [i16]]) {
    SYNTHESIZER.lock().unwrap().run(buffers);
}

fn play_sound_short() {
    use stream::{Format, Stream};

    println!("Play `short` stream");
    let mut stm = Stream::new(CHANNELS, Format::S16LE, RATE, fill_buffer_short).unwrap();
    stm.init().unwrap();
    stm.start().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    stm.stop().unwrap();
}

fn main() {
    play_sound_float();
    play_sound_short();
    show_result();
    change_default_devices();
}
