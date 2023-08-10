use serde_derive::{Deserialize, Serialize};

pub mod gate_mode_data {
    pub const AUTO: u8 = 1;
    pub const EXTERNAL: u8 = 2;
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct GateMode {
    pub data: u8,
}
