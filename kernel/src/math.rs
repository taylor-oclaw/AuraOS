//! Math functions for no_std environment
//! Wraps libm for bare-metal use

pub fn sqrtf(x: f32) -> f32 { libm::sqrtf(x) }
pub fn expf(x: f32) -> f32 { libm::expf(x) }
pub fn powf(base: f32, exp: f32) -> f32 { libm::powf(base, exp) }
pub fn cosf(x: f32) -> f32 { libm::cosf(x) }
pub fn sinf(x: f32) -> f32 { libm::sinf(x) }
pub fn fabsf(x: f32) -> f32 { libm::fabsf(x) }
pub fn logf(x: f32) -> f32 { libm::logf(x) }
