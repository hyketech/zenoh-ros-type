use crate::std_msgs;
use crate::geometry_msgs;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Odometry {
    pub header: std_msgs::Header,
    pub child_frame_id: String,
    pub pose: geometry_msgs::PoseWithCovariance,
    pub twist: geometry_msgs::TwistWithCovariance,
}