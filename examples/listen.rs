use dotenv::dotenv;
use hues::{Bridge, LightCommand};
use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bridge = Bridge::new([10u8, 0, 0, 143], std::env::var("APP_KEY").unwrap())
        .listen(|changes| {
            dbg!(changes);
        })
        .await;

    for light in bridge.lights() {
        if light.supports_color() {
            let _ = light
                .send(&[LightCommand::color_from_hex("#220099").unwrap()])
                .await;
        } else {
            let _ = light.identify().await;
        }
        std::thread::sleep(Duration::from_secs(1));
    }

    loop {}
}
