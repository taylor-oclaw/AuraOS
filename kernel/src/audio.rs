extern crate alloc;

use alloc::vec::Vec;
use core::ptr;

#[repr(C)]
pub struct AC97 {
    mmio_base: *mut u8,
}

impl AC97 {
    pub const NAM_RESET: u16 = 0x00;
    pub const NAM_MASTER_VOL: u16 = 0x02;
    pub const NAM_PCM_VOL: u16 = 0x18;
    pub const NABM_PIBD: u16 = 0x00;
    pub const NABM_GLOB_CNT: u16 = 0x2C;
    pub const NABM_GLOB_STA: u16 = 0x30;
    pub const NABM_PO_BASE: u16 = 0x10;
    pub const NABM_PO_CUR: u16 = 0x14;
    pub const NABM_PO_LVI: u16 = 0x15;
    pub const NABM_PO_SR: u16 = 0x16;
    pub const NABM_PO_CR: u16 = 0x1B;

    pub fn new(base: *mut u8) -> Self {
        AC97 { mmio_base: base }
    }

    pub unsafe fn init(&self) {
        self.write_register(Self::NAM_RESET, 0x01);
        core::hint::black_box(());
        while self.read_register(Self::NAM_RESET) & 0x01 != 0 {}

        self.write_register(Self::NAM_MASTER_VOL, 0x8080);
        self.write_register(Self::NAM_PCM_VOL, 0x8080);
    }

    pub unsafe fn play_tone(&self, freq_hz: u32, duration_ms: u32) {
        let sample_rate = 48000;
        let num_samples = (sample_rate * duration_ms / 1000) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let value = (crate::math::sinf(t * freq_hz as f32 * core::f32::consts::PI * 2.0) * 127.0) as i8;
            samples.push(value);
        }

        self.write_register(Self::NABM_PO_BASE, 0x4000);
        self.write_register(Self::NABM_PO_LVI, (num_samples - 1) as u16);

        ptr::copy(samples.as_ptr(), self.mmio_base.offset(0x4000) as *mut i8, num_samples);

        self.write_register(Self::NABM_GLOB_CNT, 0x02);
        self.write_register(Self::NABM_PO_CR, 0x91);
    }

    unsafe fn read_register(&self, reg: u16) -> u16 {
        ptr::read_volatile(self.mmio_base.offset(reg as isize * 2) as *const u16)
    }

    unsafe fn write_register(&self, reg: u16, value: u16) {
        ptr::write_volatile(self.mmio_base.offset(reg as isize * 2) as *mut u16, value);
    }
}

pub fn detect_from_pci(devices: &[(u32, u32)]) -> Option<*mut u8> {
    for &(class, base) in devices {
        if class == 0x040100 {
            return Some(base as *mut u8);
        }
    }
    None
}