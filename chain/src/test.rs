use crate::time_point::TimePoint;
use crate::{Action, AmountLimit, Tx};
use base58::{FromBase58, ToBase58};
use chrono::{DateTime, TimeZone, Timelike, Utc};
use keys::algorithm;

#[test]
fn it_works() {
    let action = Action::new(
        String::from("iost"),
        String::from("transfer"),
        String::from(r#"["iost","admin","lispczz","10.12034123",""]"#),
    );
    let amount_limit = AmountLimit {
        token: "*".to_string(),
        value: "unlimited".to_string(),
    };
    let time = Utc::now().nanosecond();
    let tx = Tx {
        time: TimePoint::now().as_i64(),
        expiration: 0,
        gas_ratio: 100.0,
        gas_limit: 6000000.0,
        delay: 0,
        chain_id: 1024,
        actions: vec![action],
        amount_limit: vec![amount_limit],
        publisher: "".to_string(),
        publisher_sigs: vec![],
        signers: vec![],
        signatures: vec![],
    };

    dbg!(time);
}
