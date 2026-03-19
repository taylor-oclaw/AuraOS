//! Keyboard buffer — interrupt handler puts keys here, main loop drains

use core::sync::atomic::{AtomicUsize, Ordering};

const BUF_SIZE: usize = 64;

static mut KEY_BUF: [u8; BUF_SIZE] = [0; BUF_SIZE];
static WRITE_POS: AtomicUsize = AtomicUsize::new(0);
static READ_POS: AtomicUsize = AtomicUsize::new(0);

/// Called from interrupt handler — push a key into the buffer
pub fn push_key(key: u8) {
    let wp = WRITE_POS.load(Ordering::Relaxed);
    let next = (wp + 1) % BUF_SIZE;
    let rp = READ_POS.load(Ordering::Relaxed);
    if next != rp {
        unsafe { KEY_BUF[wp] = key; }
        WRITE_POS.store(next, Ordering::Release);
    }
    // else: buffer full, drop key
}

/// Called from main loop — pop a key if available
pub fn pop_key() -> Option<u8> {
    let rp = READ_POS.load(Ordering::Relaxed);
    let wp = WRITE_POS.load(Ordering::Acquire);
    if rp == wp {
        None
    } else {
        let key = unsafe { KEY_BUF[rp] };
        READ_POS.store((rp + 1) % BUF_SIZE, Ordering::Release);
        Some(key)
    }
}
