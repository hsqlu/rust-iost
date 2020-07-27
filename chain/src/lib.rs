#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use self::action::*;
use crate::bytes::*;
pub use crate::error::Error;
pub use crate::get_batch_contract_storage::{BatchContractStorage, BatchContractStoragePost};
pub use crate::get_chain_info::ChainInfo;
pub use crate::get_contract_storage::{ContractStorage, ContractStoragePost};
pub use crate::get_contract_storage_fields::{ContractStorageFields, ContractStorageFieldsPost};
pub use crate::get_gas_ratio::GasRatio;
pub use crate::get_node_info::NodeInfo;
pub use crate::get_ram_info::RamInfo;
pub use crate::key_field::KeyField;
pub use crate::message::ErrorMessage;
use crate::tx::Tx;
use crate::tx_response::TxResponse;
use alloc::format;
use alloc::string::String;
#[cfg(feature = "std")]
use async_trait::async_trait;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub mod abi;
pub mod action;
pub mod amount_limit;
pub mod block;
pub mod bytes;
pub mod error;
pub mod fixed;
pub mod frozen_balance;
pub mod gas_info;
pub mod get_account;
pub mod get_batch_contract_storage;
pub mod get_block_by_hash;
pub mod get_candidate_bonus;
pub mod get_chain_info;
pub mod get_contract;
pub mod get_contract_storage;
pub mod get_contract_storage_fields;
pub mod get_gas_ratio;
pub mod get_node_info;
pub mod get_producer_vote_info;
pub mod get_ram_info;
pub mod get_token_balance;
pub mod get_token_info;
pub mod get_tx_by_hash;
pub mod get_voter_bonus;
pub mod group;
pub mod info;
pub mod item;
pub mod key_field;
pub mod message;
pub mod names;
pub mod net_work_info;
pub mod permission;
pub mod pledge_info;
pub mod ram_info;
pub mod receipts;
pub mod signature;
pub mod status;
pub mod status_code;
pub mod transaction;
pub mod tx;
pub mod tx_receipt;
pub mod tx_response;
pub mod unsigned_int;
pub mod vote_info;

pub use iost_derive::*;

pub use self::{action::*, bytes::*, error::*, names::*, unsigned_int::*};

struct IOST {
    host: String,
    client: reqwest::Client,
}

#[async_trait]
trait Client {
    fn new(host: &str) -> Self;

    async fn get<T>(&self, path: &str) -> core::result::Result<T, Error>
    where
        T: 'static + for<'de> Deserialize<'de>;

    async fn post<T, R>(&self, path: &str, param: R) -> core::result::Result<T, Error>
    where
        T: 'static + for<'de> Deserialize<'de>,
        R: Serialize + Send + Sync;
}

#[async_trait]
impl Client for IOST {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    async fn get<T>(&self, path: &str) -> core::result::Result<T, Error>
    where
        T: 'static + for<'de> Deserialize<'de>,
    {
        let url = format!("{}/{}", self.host, path);
        let response = self.client.get(&url).send().await.map_err(Error::Reqwest)?;
        if response.status() == 200 {
            let result = response.json::<T>().await.map_err(Error::Reqwest)?;
            Ok(result)
        } else {
            let rsp = response
                .json::<ErrorMessage>()
                .await
                .map_err(Error::Reqwest)?;
            Err(Error::ErrorMessage(rsp))
        }
    }

    async fn post<T, R>(&self, path: &str, param: R) -> core::result::Result<T, Error>
    where
        T: 'static + for<'de> Deserialize<'de>,
        R: Serialize + Send + Sync,
    {
        let url = format!("{}/{}", self.host, path);
        let req = reqwest::Client::new()
            .post(&url)
            .json(&param)
            .send()
            .await
            .map_err(Error::Reqwest)?;
        let code_status = req.status();
        if code_status == 200 {
            let response = req.json().await.map_err(Error::Reqwest)?;
            Ok(response)
        } else {
            let response = req.json().await.map_err(Error::Reqwest)?;
            Err(Error::ErrorMessage(response))
        }
    }
}

impl IOST {
    pub async fn get_node_info(&self) -> core::result::Result<NodeInfo, Error> {
        self.get("getNodeInfo").await
    }

    pub async fn get_chain_info(&self) -> core::result::Result<ChainInfo, Error> {
        self.get("getChainInfo").await
    }

    pub async fn get_gas_ratio(&self) -> core::result::Result<GasRatio, Error> {
        self.get("getGasRatio").await
    }

    pub async fn get_ram_info(&self) -> core::result::Result<RamInfo, Error> {
        self.get("getRAMInfo").await
    }

    pub async fn get_contract_storage(
        &self,
        par: ContractStoragePost,
    ) -> core::result::Result<ContractStorage, Error> {
        self.post("getContractStorage", &par).await
    }

    pub async fn get_contract_storage_fields(
        &self,
        par: ContractStorageFieldsPost,
    ) -> core::result::Result<ContractStorageFields, Error> {
        self.post("getContractStorageFields", &par).await
    }

    pub async fn get_batch_contract_storage(
        &self,
        par: BatchContractStoragePost,
    ) -> core::result::Result<BatchContractStorage, Error> {
        self.post("getBatchContractStorage", &par).await
    }

    pub async fn send_tx(&self, par: Tx) -> core::result::Result<TxResponse, Error> {
        self.post("sendTx", par).await
    }
}

/// Execute test file command "cargo test iost_basic_test -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;
    use crate::amount_limit::AmountLimit;
    use crate::signature::Signature;

    #[tokio::test]
    async fn iost_basic_get_method_should_be_ok() {
        let host = "https://api.iost.io";
        let iost = IOST::new(host);
        let result = iost.get_node_info().await;
        assert!(result.is_ok());
        let chain_result = iost.get_chain_info().await;
        assert!(chain_result.is_ok());
        let gas_result = iost.get_gas_ratio().await;
        assert!(gas_result.is_ok());
        let ram_result = iost.get_ram_info().await;
        assert!(ram_result.is_ok());
    }

    #[tokio::test]
    async fn iost_basic_post_method_should_be_ok() {
        let host = "https://api.iost.io";
        let iost = IOST::new(host);
        let new_post = ContractStoragePost {
            id: "token.iost".to_string(),
            key: "TIiost".to_string(),
            field: "decimal".to_string(),
            by_longest_chain: true,
        };
        let result = iost.get_contract_storage(new_post).await;
        assert!(result.is_ok());
        let new_post = ContractStorageFieldsPost {
            id: "token.iost".to_string(),
            key: "TIiost".to_string(),
            by_longest_chain: true,
        };
        let field_result = iost.get_contract_storage_fields(new_post).await;
        assert!(field_result.is_ok());
        let key = KeyField {
            key: "supply".to_string(),
            field: "TIiost".to_string(),
        };

        let key1 = KeyField {
            key: "decimal".to_string(),
            field: "TIiost".to_string(),
        };

        let posts = BatchContractStoragePost {
            id: "token.iost".to_string(),
            key_fields: vec![key, key1],
            by_longest_chain: true,
        };
        let storage_result = iost.get_batch_contract_storage(posts).await;
        assert!(storage_result.is_ok());
    }

    #[tokio::test]
    async fn test_send_tx_should_be_ok() {
        let host = "https://api.iost.io";
        let iost = IOST::new(host);
        let action = Action{
            contract: "token.iost".to_string(),
            action_name: "transfer".to_string(),
            data: "[\"iost\", \"testaccount\", \"anothertest\", \"100\", \"this is an example transfer\"]".to_string()
        };

        let amount_limit = AmountLimit {
            token: "*".to_string(),
            value: "unlimited".to_string(),
        };

        let signature = Signature{
            algorithm: "ED25519".to_string(),
            signature: "lDS+SdM+aiVHbDyXapvrsgyKxFg9mJuHWPZb/INBRWY=".to_string(),
            public_key: "/K1HM0OEbfJ4+D3BmalpLmb03WS7BeCz4nVHBNbDrx3/A31aN2RJNxyEKhv+VSoWctfevDNRnL1kadRVxSt8CA==".to_string()
        };

        let tx = Tx {
            time: 1544709662543340000,
            expiration: 1544709692318715000,
            gas_ratio: 1.0,
            gas_limit: 500000.0,
            delay: 0,
            chain_id: 1024,
            actions: vec![action],
            amount_limit: vec![amount_limit],
            publisher: "testaccount".to_string(),
            publisher_sigs: vec![signature],
            signers: vec![],
            signatures: vec![],
        };

        let tx_result = iost.send_tx(tx).await;
        dbg!(tx_result);
    }
}
