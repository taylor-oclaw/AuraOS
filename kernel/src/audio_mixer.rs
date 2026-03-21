extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct AudioSample {
    pub data: Vec<i16>,
    pub sample_rate: u32,
    pub channels: u8
}

pub struct AudioChannel {
    pub name: String,
    pub volume: u8,
    pub muted: bool,
    pub samples: Vec<i16>,
    pub position: usize
}

pub struct AudioMixer {
    pub channels: Vec<AudioChannel>,
    pub master_volume: u8,
    pub sample_rate: u32,
    pub output_buffer: Vec<i16>,
    pub buffer_size: usize
}

impl AudioMixer {
    pub fn new(sample_rate: u32, buffer_size: usize) -> Self {
        Self {
            channels: Vec::new(),
            master_volume: 80,
            sample_rate,
            output_buffer: vec![0; buffer_size],
            buffer_size
        }
    }

    pub fn add_channel(&mut self, name: &str) -> usize {
        let idx = self.channels.len();
        self.channels.push(AudioChannel {
            name: String::from(name),
            volume: 100,
            muted: false,
            samples: Vec::new(),
            position: 0
        };
        idx
    }

    pub fn play(&mut self, channel: usize, samples: Vec<i16>) {
        if channel < self.channels.len() {
            self.channels[channel].samples = samples;
            self.channels[channel].position = 0;
        }
    }

    pub fn set_volume(&mut self, channel: usize, vol: u8) {
        if channel < self.channels.len() {
            self.channels[channel].volume = vol.min(100);
        }
    }

    pub fn mute(&mut self, channel: usize) {
        if channel < self.channels.len() {
            self.channels[channel].muted = !self.channels[channel].muted;
        }
    }

    pub fn mix(&mut self) {
        for s in &mut self.output_buffer {
            *s = 0;
        }
        for ch in &mut self.channels {
            if ch.muted || ch.samples.is_empty() {
                continue;
            }
            let vol = ch.volume as i32 * self.master_volume as i32 / 10000;
            for i in 0..self.buffer_size {
                if ch.position < ch.samples.len() {
                    let sample = (ch.samples[ch.position] as i32 * vol) as i16;
                    if i < self.output_buffer.len() {
                        self.output_buffer[i] = self.output_buffer[i].saturating_add(sample);
                    }
                    ch.position += 1;
                }
            }
        }
    }

    pub fn is_playing(&self) -> bool {
        self.channels.iter().any(|c| !c.muted && c.position < c.samples.len())
    }
)}
