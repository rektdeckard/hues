//! A Rust client for the Philips Hue API v2.
//!
//! Provides a convenient builder-style interface to control your Philips Hue
//! home automation devices, with a focus on accuracy to the [official spec](https://developers.meethue.com/develop/hue-api-v2/).
//! It uses [reqwest](https://docs.rs/reqwest/0.11) and the [tokio](https://docs.rs/tokio/1) async runtime.
//!
//! This library is experimental, and is not yet ready for production use. It
//! currently supports most basic operations, such as:
//!
//! - Local network device discovery
//!   - Via **mDNS** using the [mdns](https://docs.rs/mdns/3) crate, requires the `mdns` feature
//!   - Via **HTTPS** using the Hue Discovery Endpoint
//! - App key creation
//! - Light, Group, and Scene control
//! - Schedule and Smart Scene management
//!
//! It does not yet support the following features:
//!
//! - [Entertainment API](https://developers.meethue.com/develop/hue-entertainment/)
//! for fast, synchronous light effects via UDP
//! - Advanced features regarding Entertainment Configurations
//!
//! # Basic usage
//!
//! If you already know your Bridge IP address and have previously created an
//! App Key, constructing a client is quick and simple:
//!
//! ```no_run
//! use hues::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), HueAPIError> {
//!     // Construct a Bridge when IP and App Key are known
//!     let bridge = Bridge::new([10u8, 0, 0, 123], "my-app-key");
//!     // Refresh it to fetch the current state of all resources
//!     bridge.refresh().await?;
//!     
//!     // Toggle the power state of a Room named "office", if it exists
//!     if let Some(office) = bridge.rooms().iter().find(|r| r.name() == "office") {
//!         office.toggle().await?;
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Bridge discovery and registration
//!
//! When the Bridge IP address is not known, you can locate the device on the
//! local network using the [Bridge::discover](service::Bridge::discover)
//! associated function. If you are creating an app for the first time, the
//! [Bridge::create_app](service::Bridge::create_app) method initializes new
//! credentials that can be used for future authentication.
//!
//! ```no_run
//! use hues::prelude::*;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Discover a Hue Bridge on the local network, and initialize polling
//!     // to synchronize state every 30 seconds.
//!     let mut bridge = Bridge::discover()
//!         .await
//!         .unwrap()
//!         .build()
//!         .poll(Duration::from_secs(30))
//!         .await;
//!     // This is your App Key, it should be saved for future sessions
//!     // NOTE: press the `Link Button` on the Hues Bridge before attempting
//!     // to create new app credentials.
//!     let key = bridge.create_app("my_app", "my_instance").await.unwrap();
//!
//!     // Blink each light to confirm you're registered!
//!     for light in bridge.lights() {
//!         let _ = light.identify().await;
//!     }
//! }
//! ```
//!
//! # Automatic sync with `sse`
//!
//! Optionally, you can sync automatically by listening for Server-Sent Events.
//! The bridge will communicate changes as they happen to the client, and you
//! can take action if you choose to do so:
//!
//! ```no_run
//! use hues::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let bridge = Bridge::new([10u8, 0, 0, 123], "my_app_key").listen(|_rids| {
//!         // Do something whenever changes are sent from the Bridge
//!     }).await;
//! }
//! ```

pub mod api;
pub mod command;
mod event;
pub mod service;

pub mod prelude {
    pub use crate::{
        api::HueAPIError,
        command::*,
        service::{Bridge, BridgeBuildError, BridgeBuilder, ResourceIdentifier, ResourceType},
    };
}
