use crate::std_msgs::Header;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct HykeThrusterCommand {
    pub i: i32, // index
    pub u: f32, // power / force
    pub alpha: f32, // angle
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HykeThrusterCommands {
    pub header: Header,
    pub commands: [HykeThrusterCommand; 3],
}