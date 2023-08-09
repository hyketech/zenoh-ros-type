use crate::builtin_interfaces::Time;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AckermannControlCommand {
    pub stamp: Time,
    pub lateral: AckermannLateralCommand,
    pub longitudinal: LongitudinalCommand,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AckermannLateralCommand {
    pub stamp: Time,
    pub steering_tire_angle: f32,
    pub steering_tire_rotation_rate: f32,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct LongitudinalCommand {
    pub stamp: Time,
    pub speed: f32,
    pub acceleration: f32,
    pub jerk: f32,
}
