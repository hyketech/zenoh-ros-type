use crate::builtin_interfaces::Time;
use crate::service::ServiceHeader;
use crate::std_msgs::StdMsgsHeader;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct GateMode {
    pub data: u8, // 0: AUTO, 1: EXTERNAL
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct GetEngage {
    pub ts: Time,
    pub enable: bool,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Engage {
    pub header: ServiceHeader,
    pub enable: bool,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct ResponseStatus {
    header: ServiceHeader,
    code: u32,
    message: String,
}

pub const GEAR_CMD_DRIVE: u8 = 2;
pub const GEAR_CMD_REVERSE: u8 = 20;
pub const GEAR_CMD_PARK: u8 = 22;
pub const GEAR_CMD_LOW: u8 = 23;
#[derive(Serialize, Deserialize, PartialEq)]
pub struct GearCommand {
    pub ts: Time,
    pub command: u8,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CurrentVelocity {
    pub header: StdMsgsHeader,
    pub longitudinal_velocity: f32,
    pub lateral_velocity: f32,
    pub heading_rate: f32,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AckermannLateralCommand {
    pub ts: Time,
    pub steering_tire_angle: f32,
    pub steering_tire_rotation_rate: f32,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct LongitudinalCommand {
    pub ts: Time,
    pub speed: f32,
    pub acceleration: f32,
    pub jerk: f32,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AckermannControlCommand {
    pub ts: Time,
    pub lateral: AckermannLateralCommand,
    pub longitudinal: LongitudinalCommand,
}
