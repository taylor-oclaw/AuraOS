//! WPA2/WPA3 authentication
//! Implements the 4-way handshake for WPA-Personal.

/// WPA authentication state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WpaState {
    Idle,
    WaitingForMessage1,
    SentMessage2,
    WaitingForMessage3,
    SentMessage4,
    Authenticated,
    Failed,
}
