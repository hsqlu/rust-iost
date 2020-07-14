use crate::message::ErrorMessage;
// #[cfg(feature = "std")]
// use serde::{Serialize, Deserialize};

#[derive(Debug)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Error {
    ///Error request message
    Reqwest(reqwest::Error),
    ///Error response message
    ErrorMessage(ErrorMessage),
}
