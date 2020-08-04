#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod algorithm;
pub mod error;
pub mod keypair;
pub mod public;
pub mod secret;
pub mod signature;

mod base58;
mod constant;
mod hash;
mod network;

use error::Result;

#[cfg(test)]
mod test {
    use super::algorithm;
    use base58::{FromBase58, ToBase58};
    use ed25519_dalek::Keypair;

    #[test]
    fn it_works() {
        // let s = "1rANSfcRzr4HkhbUFZ7L1Zp69JZZHiDDq5v7dNSbbEqeU4jxy3fszV4HGiaLQEyqVpS1dKT9g7zCVRxBVzuiUzB".from_base58().unwrap();
        // #[cfg(feature = "std")]
        let s = "1rANSfcRzr4HkhbUFZ7L1Zp69JZZHiDDq5v7dNSbbEqeU4jxy3fszV4HGiaLQEyqVpS1dKT9g7zCVRxBVzuiUzB".from_base58().unwrap();
        let pub_key = algorithm::new("ed25519").get_pub_key(s.as_ref()).unwrap();
        // dbg!(pub_key);
        // let key_pair = from_bytes::from_bytes(s.as_ref()).unwrap();
        // dbg!(key_pair.public.to_bytes());
        let result = pub_key.to_base58();
        assert_eq!("6sNQa7PV2SFzqCBtQUcQYJGGoU7XaB6R4xuCQVXNZe6b", result);


    }
}