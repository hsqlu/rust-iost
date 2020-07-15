use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Receipt {
    /// ABI function name
    pub func_name: String,
    /// content
    pub content: String,
}
