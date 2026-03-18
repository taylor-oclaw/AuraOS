//! Aura Biometric — Identity through your body
//! 
//! Multi-modal biometric authentication:
//! - Face recognition (primary — like Face ID)
//! - Voice recognition (secondary — "Hey Aura, it's me")
//! - Typing pattern (continuous — behavioral biometrics)
//! - Fingerprint (if hardware available)
//! 
//! All biometric data stored locally, encrypted, never leaves device.

/// Biometric enrollment
#[derive(Debug, Clone)]
pub struct BiometricProfile {
    pub user_id: String,
    pub enrolled_methods: Vec<BiometricMethod>,
    pub primary_method: BiometricMethod,
    pub fallback_method: AuthFallback,
    pub created_at: u64,
    pub last_used_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiometricMethod {
    Face,
    Voice,
    Fingerprint,
    TypingPattern,
    PhoneProximity,  // Phone + PIN as auth
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthFallback {
    PIN,
    Password,
    RecoveryKey,
}

/// Authentication result
#[derive(Debug, Clone)]
pub struct AuthResult {
    pub authenticated: bool,
    pub user_id: Option<String>,
    pub method: BiometricMethod,
    pub confidence: f32,
    pub anti_spoof_passed: bool, // Liveness detection
}

/// Privacy controls for biometrics
#[derive(Debug, Clone)]
pub struct BiometricPrivacy {
    /// All data encrypted at rest with hardware key
    pub encrypted: bool,
    /// Never sent to any server
    pub local_only: bool,
    /// Auto-delete after N days of non-use
    pub auto_delete_days: Option<u32>,
    /// Require PIN every N hours regardless of biometric
    pub reauth_interval_hours: u32,
    /// Lockout after N failed attempts
    pub max_failed_attempts: u8,
}

impl Default for BiometricPrivacy {
    fn default() -> Self {
        Self {
            encrypted: true,
            local_only: true,
            auto_delete_days: None,
            reauth_interval_hours: 72,
            max_failed_attempts: 5,
        }
    }
}
