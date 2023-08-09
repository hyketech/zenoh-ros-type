use crate::builtin_interfaces::Time;
use crate::service::ServiceHeader;
use crate::std_msgs::StdMsgsHeader;
use serde_derive::{Deserialize, Serialize};

pub const CONTROL_MODE_CMD_NO_COMMAND: u8 = 0;
pub const CONTROL_MODE_CMD_AUTONOMOUS: u8 = 1;
pub const CONTROL_MODE_CMD_MANUAL: u8 = 2;
#[derive(Serialize, Deserialize, PartialEq)]
pub struct ControlModeCommand {
    pub stamp: Time,
    pub mode: u8,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ControlModeReport {
    pub stamp: Time,
    pub mode: u8,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Engage {
    pub stamp: Time,
    pub enable: bool,
}

pub const GEAR_CMD_DRIVE: u8 = 2;
pub const GEAR_CMD_REVERSE: u8 = 20;
pub const GEAR_CMD_PARK: u8 = 22;
pub const GEAR_CMD_LOW: u8 = 23;
#[derive(Serialize, Deserialize, PartialEq)]
pub struct GearCommand {
    pub stamp: Time,
    pub command: u8,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct GearReport {
    pub stamp: Time,
    pub report: u8,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct VelocityReport {
    pub header: StdMsgsHeader,
    pub longitudinal_velocity: f32,
    pub lateral_velocity: f32,
    pub heading_rate: f32,
}

// -----service-----

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ControlModeCommandRequest {
    pub header: ServiceHeader,
    pub mode: bool,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ControlModeCommandResponse {
    pub header: ServiceHeader,
    pub code: u32,
    pub message: String,
    pub success: bool,
}
