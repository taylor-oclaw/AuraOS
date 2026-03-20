extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod chacha20 {
    use super::*;

    pub struct ChaCha20 {
        key: [u8; 32],
        nonce: [u8; 12],
        counter: u32,
    }

    impl ChaCha20 {
        pub fn new(key: &[u8], nonce: &[u8]) -> Self {
            let mut k = [0u8; 32];
            let mut n = [0u8; 12];
            k.copy_from_slice(&key[..32]);
            n.copy_from_slice(&nonce[..12]);
            ChaCha20 { key: k, nonce: n, counter: 0 }
        }

        fn quarter_round(x: &mut [u32]) {
            x[0] = x[0].wrapping_add(x[1]); x[3] ^= x[0]; x[3] = x[3].rotate_left(16);
            x[2] = x[2].wrapping_add(x[3]); x[1] ^= x[2]; x[1] = x[1].rotate_left(12);
            x[0] = x[0].wrapping_add(x[1]); x[3] ^= x[0]; x[3] = x[3].rotate_left(8);
            x[2] = x[2].wrapping_add(x[3]); x[1] ^= x[2]; x[1] = x[1].rotate_left(7);
        }

        fn double_round(state: &mut [u32]) {
            ChaCha20::quarter_round(&mut state[0..4]);
            ChaCha20::quarter_round(&mut state[4..8]);
            ChaCha20::quarter_round(&mut state[8..12]);
            ChaCha20::quarter_round(&mut state[12..16]);

            ChaCha20::quarter_round(&mut [state[0], state[5], state[10], state[15]]);
            ChaCha20::quarter_round(&mut [state[1], state[6], state[11], state[12]]);
            ChaCha20::quarter_round(&mut [state[2], state[7], state[8], state[13]]);
            ChaCha20::quarter_round(&mut [state[3], state[4], state[9], state[14]]);
        }

        fn block(&self) -> [u32; 16] {
            let mut state = [
                0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
                u32::from_le_bytes(self.key[0..4].try_into().unwrap()),
                u32::from_le_bytes(self.key[4..8].try_into().unwrap()),
                u32::from_le_bytes(self.key[8..12].try_into().unwrap()),
                u32::from_le_bytes(self.key[12..16].try_into().unwrap()),
                u32::from_le_bytes(self.key[16..20].try_into().unwrap()),
                u32::from_le_bytes(self.key[20..24].try_into().unwrap()),
                u32::from_le_bytes(self.key[24..28].try_into().unwrap()),
                u32::from_le_bytes(self.key[28..32].try_into().unwrap()),
                u32::from_le_bytes(self.nonce[0..4].try_into().unwrap()),
                u32::from_le_bytes(self.nonce[4..8].try_into().unwrap()),
                u32::from_le_bytes(self.nonce[8..12].try_into().unwrap()),
                self.counter,
            ];

            for _ in 0..10 {
                ChaCha20::double_round(&mut state);
            }

            let mut output = [0u32; 16];
            for i in 0..16 {
                output[i] = state[i].wrapping_add(state[i + 16]);
            }
            output
        }

        pub fn encrypt(&mut self, plaintext: &[u8]) -> Vec<u8> {
            let mut ciphertext = Vec::with_capacity(plaintext.len());
            for chunk in plaintext.chunks(64) {
                let block = self.block();
                for (i, &byte) in chunk.iter().enumerate() {
                    ciphertext.push(byte ^ ((block[i / 4] >> ((i % 4) * 8)) as u8));
                }
                self.counter += 1;
            }
            ciphertext
        }

        pub fn decrypt(&mut self, ciphertext: &[u8]) -> Vec<u8> {
            let mut plaintext = Vec::with_capacity(ciphertext.len());
            for chunk in ciphertext.chunks(64) {
                let block = self.block();
                for (i, &byte) in chunk.iter().enumerate() {
                    plaintext.push(byte ^ ((block[i / 4] >> ((i % 4) * 8)) as u8));
                }
                self.counter += 1;
            }
            plaintext
        }

        pub fn set_nonce(&mut self, nonce: &[u8]) {
            let mut n = [0u8; 12];
            n.copy_from_slice(&nonce[..12]);
            self.nonce = n;
        }

        pub fn get_counter(&self) -> u32 {
            self.counter
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chacha20() {
        let key = b"an example very very secret key!";
        let nonce = b"a unique nonce";
        let plaintext = b"some plaintext message";

        let mut chacha = chacha20::ChaCha20::new(key, nonce);
        let ciphertext = chacha.encrypt(plaintext);

        assert_ne!(ciphertext, plaintext);

        let mut chacha_decrypt = chacha20::ChaCha20::new(key, nonce);
        let decrypted_text = chacha_decrypt.decrypt(&ciphertext);

        assert_eq!(decrypted_text, plaintext.to_vec());
    }
}
