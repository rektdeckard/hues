use dotenv::dotenv;
use hues::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut bridge = Bridge::discover()
        .await
        .unwrap()
        .build()
        .poll(Duration::from_secs(30))
        .await;
    let key = bridge.create_app("my_app", "my_instance").await.unwrap();

    for light in bridge.lights() {
        let _ = light.identify().await;
    }

    // KNOWN IP AND APP
    if true {
        // If you know your IP and App Key in advance:

        let bridge = Bridge::new([10u8, 0, 0, 143], std::env::var("APP_KEY").unwrap())
            // This initiates polling. Polling updates all device states periodically.
            // Without either polling or calling `bridge.refresh().await`,
            // no devices will be populated on the bridge.
            .poll(Duration::from_secs(30))
            .await;
    }

    // DISCOVER BRIDGE WITH EXISTING APP
    if false {
        // If you know your App Key, but not what IP your bridge has:

        let bridge = Bridge::discover()
            .await
            .unwrap()
            .app_key(&std::env::var("APP_KEY").unwrap())
            .build();
    }

    // DISCOVER BRIDGE AND CREATE APP
    if false {
        // If you do not know your bridge IP, and have not yet created an app key
        // NOTE: you should only do this once. Some settings and configurations
        // are tied to your app key, so you won't be able to recall them from
        // another app key.

        let mut bridge = Bridge::discover().await.unwrap().build();

        // This is your App Key, save it for later!

        let key = bridge
            .create_app("my_hue_app", "my_instance_name")
            .await
            .unwrap()
            .to_owned();
    }
}
