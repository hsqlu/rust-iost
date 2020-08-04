use crate::error::Error::ErrorEd25519;
use crate::Result;
use rand::rngs::OsRng;

const ROOT_KEY: &'static str =
    "2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1";

pub trait Algorithm {
    fn sign(&self, message: &[u8], sec_key: &[u8]) -> &[u8];
    fn verify(&self, message: &[u8], pub_key: &[u8], signature: &[u8]) -> bool;
    fn gen_sec_key(&self) -> Vec<u8>;
    fn get_pub_key(&self, sec_key: &[u8]) -> crate::Result<Vec<u8>>;
    fn check(&self, sec_key: &[u8]) -> crate::Result<()>;
}

pub struct AlgorithmSecp256k1;
pub struct AlgorithmEd25519;

pub fn new(algorithm_name: &'static str) -> Box<dyn Algorithm> {
    match algorithm_name {
        "ed25519" => Box::new(AlgorithmEd25519),
        "Secp256k1" => Box::new(AlgorithmSecp256k1),
        _ => Box::new(AlgorithmSecp256k1),
    }
}

impl Algorithm for AlgorithmEd25519 {
    fn sign(&self, message: &[u8], sec_key: &[u8]) -> &[u8] {
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
        // let pub_key: ed25519_dalek::PublicKey = ;
        Ok(Vec::from(key_pair.public.as_ref()))
    }

    fn check(&self, sec_key: &[u8]) -> crate::Result<()> {
        Ok(())
    }
}

impl Algorithm for AlgorithmSecp256k1 {
    fn sign(&self, message: &[u8], sec_key: &[u8]) -> &[u8] {
        unimplemented!()
    }

    fn verify(&self, message: &[u8], pub_key: &[u8], signature: &[u8]) -> bool {
        unimplemented!()
    }

    fn gen_sec_key(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn get_pub_key(&self, sec_key: &[u8]) -> Result<Vec<u8>> {
        unimplemented!()
    }

    fn check(&self, sec_key: &[u8]) -> Result<()> {
        unimplemented!()
    }
}
