#![allow(dead_code)]

use alloc::collections::btree_map::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::error::Error;
use crate::message::ErrorMessage;
use crate::receipts::Receipt;
use crate::status_code::StatusCode;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TxReceipt {
    /// hash of the transaction
    pub tx_hash: String,
    /// GAS consumption of the transaction
    pub gas_usage: f64,
    /// RAM consumption for the transaction. map-key is account name, and value is RAM amount
    pub ram_usage: BTreeMap<String, i64>,
    /// Status of the transaction. SUCCESS; GAS_RUN_OUT - insufficient GAS;
    /// BALANCE_NOT_ENOUGH - insufficient balance; WRONG_PARAMETER;
    /// RUNTIME_ERROR - a run-time error; TIMEOUT; WRONG_TX_FORMAT;
    /// DUPLICCATE_SET_CODE - set code is duplicated unexpectedly;
    /// UNKNOWN_ERROR
    pub status_code: StatusCode,
    /// a message descripting status_code
    pub message: String,
    /// return values for each Action
    pub returns: Vec<String>,
    /// for event functions
    pub receipts: Vec<Receipt>,
}

async fn get_tx_receipt_json(domain: &str, hash: &str) -> Result<TxReceipt, Error> {
    let url = format!("{}/getTxReceiptByTxHash/{}", domain, hash);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<TxReceipt>().await.map_err(Error::Reqwest)?;
        Ok(rsp)
    } else {
        let rsp = req.json::<ErrorMessage>().await.map_err(Error::Reqwest)?;
        Err(Error::ErrorMessage(rsp))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_tx_receipt_json_should_be_ok() {
        let response = get_tx_receipt_json(
            "http://api.iost.io",
            "Dj8bmA4Fx4LHrwLtDB6EEkNbBFU8biENxf55mNaJewYw",
        )
        .await;
        assert!(response.is_ok());
    }
}
