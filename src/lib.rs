//! Rust bindings for the Philips Hue API v2.
//!
//! Provides a convenient builder-style interface to controlling your
//! home automation devices, with easy accees to:
//! - Local network device discovery
//! - App key creation
//! - Group and behavior

pub mod api;
mod command;
mod event;
mod service;

pub use command::*;
pub use service::behavior::*;
pub use service::bridge::*;
pub use service::control::*;
pub use service::device::*;
pub use service::group::*;
pub use service::light::*;
pub use service::resource::*;
pub use service::scene::*;
pub use service::sensor::*;
pub use service::thirdparty::*;
pub use service::zigbee::*;
pub use service::zone::*;
