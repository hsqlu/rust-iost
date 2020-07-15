use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    PENDING,
    PACKED,
    IRREVERSIBLE,
    APPROVED,
}
