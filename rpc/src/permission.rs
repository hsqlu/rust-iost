use crate::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    /// permission name
    pub name: String,
    /// permission group names
    pub group_names: Vec<String>,
    /// permission information
    pub items: Vec<Item>,
    /// permission threshold
    pub threshold: String,
}
