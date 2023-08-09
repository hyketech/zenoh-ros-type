//! # zenoh-ros-type
//!
//! Common Rust struct for ROS 2 messages used by Zenoh.
//! We can communicate to the ROS application behind zenoh-bridge-dds with Zenoh Rust API.
//!
//! Here are some ROS message source:
//! * [common_interface](https://github.com/ros2/common_interfaces): Common-used ROS message
//! * [rcl_interface](https://github.com/ros2/rcl_interfaces): Common interface in RCL
//! * [tier4_autoware_msgs](https://github.com/tier4/tier4_autoware_msgs/tree/tier4/universe): The
//! messages used in Autoware

pub mod autoware_auto_control_msgs;
pub mod autoware_auto_vehicle_msgs;
pub mod builtin_interfaces;
pub mod geometry_msgs;
pub mod rosgraph_msgs;
pub mod service;
pub mod std_msgs;
