use crate::message::ErrorMessage;
use crate::ParseNameError;
// #[cfg(feature = "std")]
// use serde::{Serialize, Deserialize};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Error {
    ///Error request message
    Reqwest(reqwest::Error),
    ///Error response message
    ErrorMessage(ErrorMessage),

    ParseNameErr(ParseNameError),

    FixedParseOverflow(),
    FixedParseAbnormalChar(),
    FixedParseAmountFormat(),
    FixedParseDivideByZero(),
    FixedParseDoubleDot(),
}
