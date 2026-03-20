extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod ed25519 {
    use core::convert::TryFrom;
    use curve25519_dalek::digest::{Digest, Sha512};
    use curve25519_dalek::edwards::EdwardsPoint;
    use curve25519_dalek::scalar::Scalar;
    use curve25519_dalek::signatures::{Signature, Signer, Verifier};
    use curve25519_dalek::constants::ED25519_BASEPOINT_TABLE;
    use sha3::Sha3_512;

    pub struct SignatureEd25519 {
        keypair: ed25519_dalek::Keypair,
    }

    impl SignatureEd25519 {
        pub fn new(secret_key: &[u8]) -> Self {
            let secret_scalar = Scalar::from_bytes_mod_order(*array_ref!(secret_key, 0, 32));
            let public_point = &ED25519_BASEPOINT_TABLE * &secret_scalar;
            let keypair = ed25519_dalek::Keypair {
                secret: ed25519_dalek::SecretKey(secret_scalar.to_bytes()),
                public: ed25519_dalek::PublicKey(public_point.compress().to_bytes()),
            };
            SignatureEd25519 { keypair }
        }

        pub fn sign(&self, message: &[u8]) -> Vec<u8> {
            let signature = self.keypair.sign(message);
            signature.to_vec()
        }

        pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
            let public_key = ed25519_dalek::PublicKey::from_bytes(self.keypair.public.as_bytes()).unwrap();
            let signature = ed25519_dalek::Signature::try_from(signature).unwrap();
            public_key.verify(message, &signature).is_ok()
        }

        pub fn get_public_key(&self) -> Vec<u8> {
            self.keypair.public.as_bytes().to_vec()
        }

        pub fn get_secret_key(&self) -> Vec<u8> {
            self.keypair.secret.to_bytes().to_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ed25519::SignatureEd25519;

    #[test]
    fn test_signature() {
        let secret_key = [0u8; 32];
        let signer = SignatureEd25519::new(&secret_key);
        let message = b"Hello, world!";
        let signature = signer.sign(message);

        assert!(signer.verify(message, &signature));
    }

    #[test]
    fn test_invalid_signature() {
        let secret_key = [0u8; 32];
        let signer = SignatureEd25519::new(&secret_key);
        let message = b"Hello, world!";
        let signature = vec![0u8; 64];

        assert!(!signer.verify(message, &signature));
    }
}
