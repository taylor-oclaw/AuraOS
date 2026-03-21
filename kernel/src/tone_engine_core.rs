extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn tone_engine_core_init() {
    // Initialization logic for the tone engine core module
}

#[no_mangle]
pub extern "C" fn tone_engine_core_exit() {
    // Cleanup logic for the tone engine core module
}

pub struct ToneEngineCore {
    samples: Vec<f32>,
    sample_rate: u32,
    frequency: f32,
    amplitude: f32,
    phase: f32,
}

impl ToneEngineCore {
    pub fn new(sample_rate: u32, frequency: f32, amplitude: f32) -> Self {
        ToneEngineCore {
            samples: Vec::new(),
            sample_rate,
            frequency,
            amplitude,
            phase: 0.0,
        }
    }

    pub fn generate_sine_wave(&mut self, duration_seconds: f32) {
        let num_samples = (self.sample_rate as f32 * duration_seconds) as usize;
        self.samples.clear();
        for _ in 0..num_samples {
            let sample = self.amplitude * (2.0 * core::f32::consts::PI * self.frequency * self.phase).sin();
            self.samples.push(sample);
            self.phase += 1.0 / self.sample_rate as f32;
        }
    }

    pub fn get_sample(&self, index: usize) -> Option<f32> {
        self.samples.get(index).cloned()
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude.clamped(0.0, 1.0);
    }

    pub fn get_samples(&self) -> &[f32] {
        &self.samples
    }
}
