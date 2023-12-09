//! Rust bindings for the Philips Hue API v2.
//!
//! Provides a convenient builder-style interface to controlling your
//! home automation devices, with easy accees to:
//! - Local network device discovery
//! - App key creation
//! - Group and behavior

pub mod api;
mod bridge;
mod command;
mod device;
mod group;
mod light;

pub use bridge::Bridge;
pub use command::*;
pub use device::Device;
pub use light::Light;
