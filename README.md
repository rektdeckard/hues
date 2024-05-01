# hues

A Rust client for the Philips Hue API v2, with a focus on accuracy to the [official spec](https://developers.meethue.com/develop/hue-api-v2/).

![Crates.io (version)](https://img.shields.io/crates/v/hues.svg?style=flat-square)

[![GitHub stars](https://img.shields.io/github/stars/rektdeckard/hues?style=flat-square&label=Star)](https://github.com/rektdeckard/hues)
[![GitHub forks](https://img.shields.io/github/forks/rektdeckard/hues?style=flat-square&label=Fork)](https://github.com/rektdeckard/hues/fork)
[![GitHub watchers](https://img.shields.io/github/watchers/rektdeckard/hues?style=flat-square&label=Watch)](https://github.com/rektdeckard/hues)
[![Follow on GitHub](https://img.shields.io/github/followers/rektdeckard?style=flat-square&label=Follow)](https://github.com/rektdeckard)

`hues` uses [reqwest](https://docs.rs/reqwest/0.11) and the [tokio](https://docs.rs/tokio/1) async runtime. It currently supports most basic operations, such as:

- Local network device discovery
  - Via **mDNS** using the [mdns](https://docs.rs/mdns/3) crate, requires the `mdns` feature
  - Via **HTTPS** using the Hue Discovery Endpoint
- App key creation
- Light, Group, and Scene control
- Schedule and Smart Scene management

It does not yet support the following features:

 - [Entertainment API](https://developers.meethue.com/develop/hue-entertainment/) for fast, synchronous light effects via UDP
- Advanced features regarding Entertainment Configurations


> [!WARNING]
> This is an experimental library, and is subject to change. Use at your own risk. 

## Installation

```bash
cargo add hues
```

## Usage


If you already know your Bridge IP address and have previously created an
App Key, constructing a client is quick and simple:

```rs
use hues::prelude::*;

#[tokio::main]
async fn main() -> Result<(), HueAPIError> {
    // Construct a Bridge when IP and App Key are known
    let bridge = Bridge::new([10u8, 0, 0, 123], "my-app-key");
    // Refresh it to fetch the current state of all resources
    bridge.refresh().await?;
    
    // Toggle the power state of a Room named "office", if it exists
    if let Some(office) = bridge.rooms().iter().find(|r| r.name() == "office") {
        office.toggle().await?;
    }
    Ok(())
}
```

### Bridge discovery and registration

When the Bridge IP address is not known, you can locate the device on the
local network using the [Bridge::discover](service::Bridge::discover)
associated function. If you are creating an app for the first time, the
[Bridge::create_app](service::Bridge::create_app) method initializes new
credentials that can be used for future authentication.

```rs
use hues::prelude::*
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Discover a Hue Bridge on the local network, and initialize polling
    // to synchronize state every 30 seconds.
    let mut bridge = Bridge::discover()
        .await
        .unwrap()
        .build()
        .poll(Duration::from_secs(30))
        .await;
    // This is your App Key, it should be saved for future sessions
    // NOTE: press the `Link Button` on the Hues Bridge before attempting
    // to create new app credentials.
    let key = bridge.create_app("my_app", "my_instance").await.unwrap();

    // Blink each light to confirm you're registered!
    for light in bridge.lights() {
        let _ = light.identify().await;
    }
}
```

### Automatic sync with `sse`

Optionally, you can sync automatically by listening for Server-Sent Events. The bridge will communicate changes as they happen to the client, and you can take action if you choose to do so:

```rs
use hues::prelude::*;

#[tokio::main]
async main() -> Result<(), HueAPIError> {
    let bridge = Bridge::new([10u8, 0, 0, 123], "my_app_key").listen(|_rids| {
        // Do something whenever changes are sent from the Bridge
    });
}
```

## License

MIT © [Tobias Fried](https://github.com/rektdeckard)

