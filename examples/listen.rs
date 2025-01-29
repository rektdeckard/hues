use dotenv::dotenv;
use hues::prelude::*;
use std::{net::IpAddr, time::Duration};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bridge = Bridge::new(
        std::env::var("HUE_BRIDGE_IP")
            .unwrap()
            .parse::<IpAddr>()
            .unwrap(),
        std::env::var("HUE_APP_KEY").unwrap(),
    )
    .listen(|changes| {
        dbg!(changes);
    })
    .await;

    dbg!(&bridge);

    for light in bridge.lights() {
        if light.supports_color() {
            let _ = light
                .send(&[LightCommand::color_from_hex("#c70709").unwrap()])
                .await;
        } else {
            let _ = light.identify().await;
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
