use crate::builtin_interfaces::Time;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HykeVesselCommand {
    pub ids: Vec<i32>,
    pub powers: Vec<f64>,
    pub angles: Vec<f64>,
}
