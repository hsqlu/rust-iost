use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    PENDING,
    PACKED,
    IRREVERSIBLE,
    APPROVED
}