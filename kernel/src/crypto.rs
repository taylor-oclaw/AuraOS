//! AuraOS Cryptographic Foundation
//! 
//! Pure Rust crypto — no external dependencies, no libc.
//! Implements the core primitives needed for:
//! - Disk encryption
//! - Mesh communication
//! - Biometric key derivation
//! - Permission signatures

extern crate alloc;
use alloc::vec::Vec;

// ============================================================
// SHA-256 — used for hashing, key derivation, integrity checks
// ============================================================

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    total_len: u64,
}

impl Sha256 {
    pub fn new() -> Self {
        Sha256 {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
                0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
            ],
            buffer: [0u8; 64],
            buffer_len: 0,
            total_len: 0,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut i = 0;
        self.total_len += data.len() as u64;

        // Fill buffer
        if self.buffer_len > 0 {
            let needed = 64 - self.buffer_len;
            let take = needed.min(data.len());
            self.buffer[self.buffer_len..self.buffer_len + take]
                .copy_from_slice(&data[..take]);
            self.buffer_len += take;
            i = take;

            if self.buffer_len == 64 {
                let block = self.buffer;
                self.compress(&block);
                self.buffer_len = 0;
            }
        }

        // Process full blocks
        while i + 64 <= data.len() {
            let mut block = [0u8; 64];
            block.copy_from_slice(&data[i..i + 64]);
            self.compress(&block);
            i += 64;
        }

        // Buffer remainder
        if i < data.len() {
            let rem = data.len() - i;
            self.buffer[..rem].copy_from_slice(&data[i..]);
            self.buffer_len = rem;
        }
    }

    pub fn finalize(mut self) -> [u8; 32] {
        let bit_len = self.total_len * 8;

        // Padding
        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;

        if self.buffer_len > 56 {
            for i in self.buffer_len..64 {
                self.buffer[i] = 0;
            }
            let block = self.buffer;
            self.compress(&block);
            self.buffer = [0u8; 64];
            self.buffer_len = 0;
        }

        for i in self.buffer_len..56 {
            self.buffer[i] = 0;
        }

        self.buffer[56..64].copy_from_slice(&bit_len.to_be_bytes());
        let block = self.buffer;
        self.compress(&block);

        let mut result = [0u8; 32];
        for (i, &s) in self.state.iter().enumerate() {
            result[i * 4..i * 4 + 4].copy_from_slice(&s.to_be_bytes());
        }
        result
    }

    fn compress(&mut self, block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

/// Convenience: hash a byte slice
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize()
}

// ============================================================
// AES-256 — used for disk encryption and mesh communication
// ============================================================

/// AES S-Box
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// AES Round Constants
const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

/// AES-256 key expansion
pub fn aes256_key_expand(key: &[u8; 32]) -> [[u8; 16]; 15] {
    let mut round_keys = [[0u8; 16]; 15];
    let mut expanded = [0u8; 240]; // 15 * 16 bytes

    // Copy original key
    expanded[..32].copy_from_slice(key);

    let mut i = 8; // Start after first 8 words (32 bytes)
    let mut rcon_idx = 0;

    while i < 60 {
        let mut temp = [
            expanded[(i - 1) * 4],
            expanded[(i - 1) * 4 + 1],
            expanded[(i - 1) * 4 + 2],
            expanded[(i - 1) * 4 + 3],
        ];

        if i % 8 == 0 {
            // RotWord + SubWord + Rcon
            let t = temp[0];
            temp[0] = SBOX[temp[1] as usize] ^ RCON[rcon_idx];
            temp[1] = SBOX[temp[2] as usize];
            temp[2] = SBOX[temp[3] as usize];
            temp[3] = SBOX[t as usize];
            rcon_idx += 1;
        } else if i % 8 == 4 {
            // SubWord only
            for b in &mut temp {
                *b = SBOX[*b as usize];
            }
        }

        for j in 0..4 {
            expanded[i * 4 + j] = expanded[(i - 8) * 4 + j] ^ temp[j];
        }
        i += 1;
    }

    for r in 0..15 {
        round_keys[r].copy_from_slice(&expanded[r * 16..(r + 1) * 16]);
    }

    round_keys
}

// ============================================================
// Permission System — Green/Yellow/Red tiers
// ============================================================

/// Action permission level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermissionTier {
    /// Safe actions — automatic, no confirmation
    /// Read files, display info, search, calculations
    Green,
    
    /// Potentially impactful — requires simple confirmation
    /// Modify files, change settings, install software
    Yellow,
    
    /// Dangerous/irreversible — requires biometric confirmation  
    /// Delete data, send externally, system changes, disk format
    Red,
}

/// A specific action the AI wants to perform
#[derive(Debug, Clone)]
pub struct ActionRequest {
    pub action_type: ActionType,
    pub description: alloc::string::String,
    pub tier: PermissionTier,
    pub target: alloc::string::String,
    pub approved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    // Green (automatic)
    ReadFile,
    ListFiles,
    SearchContent,
    DisplayInfo,
    Calculate,
    
    // Yellow (confirm)
    WriteFile,
    RenameFile,
    CreateFile,
    ChangeSetting,
    InstallApp,
    
    // Red (biometric)
    DeleteFile,
    FormatDisk,
    SendEmail,
    SendMessage,
    NetworkUpload,
    ModifyPermissions,
    AccessBiometrics,
    WipeDevice,
}

impl ActionType {
    pub fn tier(&self) -> PermissionTier {
        match self {
            ActionType::ReadFile | ActionType::ListFiles | ActionType::SearchContent |
            ActionType::DisplayInfo | ActionType::Calculate => PermissionTier::Green,
            
            ActionType::WriteFile | ActionType::RenameFile | ActionType::CreateFile |
            ActionType::ChangeSetting | ActionType::InstallApp => PermissionTier::Yellow,
            
            ActionType::DeleteFile | ActionType::FormatDisk | ActionType::SendEmail |
            ActionType::SendMessage | ActionType::NetworkUpload | ActionType::ModifyPermissions |
            ActionType::AccessBiometrics | ActionType::WipeDevice => PermissionTier::Red,
        }
    }
}

/// Audit log entry — every AI action is logged
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub action: ActionType,
    pub description: alloc::string::String,
    pub tier: PermissionTier,
    pub approved: bool,
    pub user_confirmed: bool,
}

/// The permission manager
pub struct PermissionManager {
    pub audit_log: Vec<AuditEntry>,
    pub auto_approve_green: bool,
    pub require_biometric_red: bool,
}

impl PermissionManager {
    pub fn new() -> Self {
        PermissionManager {
            audit_log: Vec::new(),
            auto_approve_green: true,
            require_biometric_red: true,
        }
    }
    
    /// Check if an action is allowed
    pub fn check(&self, action: &ActionRequest) -> PermissionDecision {
        match action.tier {
            PermissionTier::Green => {
                if self.auto_approve_green {
                    PermissionDecision::Approved
                } else {
                    PermissionDecision::NeedsConfirmation
                }
            }
            PermissionTier::Yellow => {
                PermissionDecision::NeedsConfirmation
            }
            PermissionTier::Red => {
                if self.require_biometric_red {
                    PermissionDecision::NeedsBiometric
                } else {
                    PermissionDecision::NeedsConfirmation
                }
            }
        }
    }
    
    /// Log an action
    pub fn log_action(&mut self, action: ActionType, description: alloc::string::String, approved: bool) {
        self.audit_log.push(AuditEntry {
            timestamp: 0, // TODO: RTC
            action,
            description,
            tier: action.tier(),
            approved,
            user_confirmed: approved,
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionDecision {
    Approved,
    NeedsConfirmation,
    NeedsBiometric,
    Denied,
}

// ============================================================
// Secure Random — using CPU's RDRAND instruction
// ============================================================

/// Generate a random u64 using hardware RNG (RDRAND)
pub fn rdrand64() -> Option<u64> {
    let mut value: u64;
    let success: u8;
    unsafe {
        core::arch::asm!(
            "rdrand {0}",
            "setc {1}",
            out(reg) value,
            out(reg_byte) success,
        );
    }
    if success != 0 { Some(value) } else { None }
}

/// Generate random bytes
pub fn random_bytes(buf: &mut [u8]) -> bool {
    let mut i = 0;
    while i < buf.len() {
        if let Some(val) = rdrand64() {
            let bytes = val.to_le_bytes();
            let remaining = buf.len() - i;
            let take = remaining.min(8);
            buf[i..i + take].copy_from_slice(&bytes[..take]);
            i += take;
        } else {
            return false; // RDRAND not available
        }
    }
    true
}

/// Generate a 256-bit random key
pub fn generate_key() -> Option<[u8; 32]> {
    let mut key = [0u8; 32];
    if random_bytes(&mut key) {
        Some(key)
    } else {
        None
    }
}
