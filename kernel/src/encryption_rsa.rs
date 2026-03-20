extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct RsaKey {
    pub modulus: u64,
    pub exponent: u64,
}

impl RsaKey {
    pub fn new(modulus: u64, exponent: u64) -> Self {
        RsaKey { modulus, exponent }
    }

    pub fn encrypt(&self, message: u64) -> u64 {
        modular_exponentiation(message, self.exponent, self.modulus)
    }

    pub fn decrypt(&self, ciphertext: u64) -> u64 {
        modular_exponentiation(ciphertext, self.exponent, self.modulus)
    }

    pub fn generate_keys(p: u64, q: u64) -> (RsaKey, RsaKey) {
        let modulus = p * q;
        let totient = (p - 1) * (q - 1);
        let exponent = find_exponent(totient);
        let d = modular_multiplicative_inverse(exponent, totient);

        let public_key = RsaKey::new(modulus, exponent);
        let private_key = RsaKey::new(modulus, d);

        (public_key, private_key)
    }

    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    pub fn exponent(&self) -> u64 {
        self.exponent
    }
}

fn modular_exponentiation(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }

    result
}

fn find_exponent(totient: u64) -> u64 {
    let mut e = 3;
    while gcd(e, totient) != 1 {
        e += 2;
    }
    e
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn modular_multiplicative_inverse(a: u64, m: u64) -> u64 {
    let (mut m0, mut x0, mut x1) = (m, 0, 1);
    if m == 1 {
        return 0;
    }
    while a > 1 {
        let q = a / m;
        let t = m;

        m = a % m;
        a = t;
        let t = x0;

        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 {
        x1 += m0;
    }
    x1
}
