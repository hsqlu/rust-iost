use crate::error::Error::{ErrorEd25519, ErrorSecp256k1};
use crate::Result;
use rand::rngs::OsRng;
#[cfg(feature = "std")]
use rand::thread_rng;

pub const ED25519: &str = "ed25519";
pub const SECP256K1: &str = "secp256k1";

const ROOT_KEY: &'static str =
    "2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1";

pub trait Algorithm {
    fn sign(&self, message: &[u8], sec_key: &[u8]) -> Vec<u8>;
    fn verify(&self, message: &[u8], pub_key: &[u8], signature: &[u8]) -> bool;
    fn gen_sec_key(&self) -> Vec<u8>;
    fn get_pub_key(&self, sec_key: &[u8]) -> crate::Result<Vec<u8>>;
    fn check(&self, sec_key: &[u8]) -> crate::Result<()>;
}

pub struct AlgorithmSecp256k1;
pub struct AlgorithmEd25519;

pub fn new(algorithm_name: &str) -> Box<dyn Algorithm> {
    match algorithm_name {
        ED25519 => Box::new(AlgorithmEd25519),
        SECP256K1 => Box::new(AlgorithmSecp256k1),
        _ => Box::new(AlgorithmSecp256k1),
    }
}

impl Algorithm for AlgorithmEd25519 {
    fn sign(&self, message: &[u8], sec_key: &[u8]) -> Vec<u8> {
        unimplemented!()
    }

    fn verify(&self, message: &[u8], pub_key: &[u8], signature: &[u8]) -> bool {
        unimplemented!()
    }

    fn gen_sec_key(&self) -> Vec<u8> {
        let mut csprng = OsRng {};
        let secret_key: ed25519_dalek::SecretKey = ed25519_dalek::SecretKey::generate(&mut csprng);
        Vec::from(secret_key.as_ref())
    }

    fn get_pub_key(&self, sec_key: &[u8]) -> crate::Result<Vec<u8>> {
        let key_pair = ed25519_dalek::Keypair::from_bytes(sec_key).map_err(ErrorEd25519)?;
        Ok(Vec::from(key_pair.public.as_ref()))
    }

    fn check(&self, sec_key: &[u8]) -> crate::Result<()> {
        Ok(())
    }
}

impl Algorithm for AlgorithmSecp256k1 {
    fn sign(&self, message: &[u8], sec_key: &[u8]) -> Vec<u8> {
        let msg = secp256k1::Message::parse_slice(message).unwrap();
        let secret_key = secp256k1::SecretKey::parse_slice(sec_key).unwrap();

        let (sig, recv_id) = secp256k1::sign(&msg, &secret_key);
        sig.serialize().to_vec()
    }

    fn verify(&self, message: &[u8], pub_key: &[u8], signature: &[u8]) -> bool {
        unimplemented!()
    }

    fn gen_sec_key(&self) -> Vec<u8> {
        let mut rng = thread_rng();
        let secret_key = secp256k1::SecretKey::random(&mut rng);
        secret_key.serialize().to_vec()
    }

    fn get_pub_key(&self, sec_key: &[u8]) -> Result<Vec<u8>> {
        let secret_key = secp256k1::SecretKey::parse_slice(sec_key).unwrap();
        let public_key = secp256k1::PublicKey::from_secret_key(&secret_key);
        Ok(public_key.serialize_compressed().to_vec())
    }

    fn check(&self, sec_key: &[u8]) -> Result<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use base58::{FromBase58, ToBase58};
    use ed25519_dalek::Keypair;

    #[test]
    fn it_works() {
        let cases = vec![
            ("2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1", "Gcv8c2tH8qZrUYnKdEEdTtASsxivic2834MQW6mgxqto"),
            ("1rANSfcRzr4HkhbUFZ7L1Zp69JZZHiDDq5v7dNSbbEqeU4jxy3fszV4HGiaLQEyqVpS1dKT9g7zCVRxBVzuiUzB", "6sNQa7PV2SFzqCBtQUcQYJGGoU7XaB6R4xuCQVXNZe6b"),
        ];

        let ed25519 = super::new("ed25519");

        for (hashed_code, expected) in cases {
            let sk = hashed_code.from_base58().unwrap();
            let pub_key = ed25519.get_pub_key(sk.as_ref()).unwrap();
            let result = pub_key.to_base58();
            assert_eq!(result, expected.to_string());
        }

        let secp256k1 = super::new("secp256k1");

        let sk = "3BZ3HWs2nWucCCvLp7FRFv1K7RR3fAjjEQccf9EJrTv4"
            .from_base58()
            .unwrap();
        let pub_key = secp256k1.get_pub_key(sk.as_ref()).unwrap();
        let result = pub_key.to_base58();
        assert_eq!("iWgLQj3VTPN4dZnomuJMMCggv22LFw4nAkA6bmrVsmCo", result);
    }
}
