//! Animation engine for smooth surface transitions
//! All animations use 60fps+ rendering with GPU acceleration.

use super::{Animation, AnimationKind, Easing};

/// Evaluate easing function at time t (0.0 to 1.0)
pub fn ease(easing: Easing, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    match easing {
        Easing::Linear => t,
        Easing::EaseIn => t * t * t,
        Easing::EaseOut => 1.0 - (1.0 - t).powi(3),
        Easing::EaseInOut => {
            if t < 0.5 {
                4.0 * t * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
            }
        }
        Easing::Spring => {
            // Damped spring oscillation
            let freq = 4.5;
            let decay = 6.0;
            1.0 - (-decay * t).exp() * (freq * core::f32::consts::PI * t).cos()
        }
        Easing::Smooth => {
            // Smooth step (Hermite interpolation)
            t * t * (3.0 - 2.0 * t)
        }
    }
}

/// Tick an animation forward by dt milliseconds
pub fn tick(anim: &mut Animation, dt_ms: u32) -> bool {
    let step = dt_ms as f32 / anim.duration_ms as f32;
    anim.progress = (anim.progress + step).min(1.0);
    anim.progress >= 1.0 // true = animation complete
}
