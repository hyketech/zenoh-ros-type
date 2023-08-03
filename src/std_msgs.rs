use crate::builtin_interfaces::Time;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct StdMsgsHeader {
    pub ts: Time,
    pub frameid: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
