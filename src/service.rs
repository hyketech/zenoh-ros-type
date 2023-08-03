use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ServiceHeader {
    pub guid: i64,
    pub seq: u64,
}
