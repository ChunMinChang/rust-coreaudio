// Introduce `lazy_static` to use a mutable global variable.
#[macro_use]
extern crate lazy_static;
// It needs to wrap a `Mutex` around the global variable to make it mutable.
use std::sync::Mutex;

extern crate rust_coreaudio;
use rust_coreaudio::stream;

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

    fn run<T>(&mut self, buffers: stream::CallbackArgs<T>)
    where
        T: std::convert::From<SynthesizedData>,
    {
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
    static ref SYNTHESIZER: Mutex<Synthesizer> =
        Mutex::new(Synthesizer::new(CHANNELS, RATE, VOLUME));
}

fn fill_buffer_float(buffers: stream::CallbackArgs<f32>) {
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

fn fill_buffer_short(buffers: stream::CallbackArgs<i16>) {
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
}
