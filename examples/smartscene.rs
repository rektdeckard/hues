use dotenv::dotenv;
use hues::{
    prelude::*,
    service::{Schedule, SmartScene, TimeslotStart, Weekday},
};
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
    .poll(Duration::from_secs(30))
    .await;

    let scenes = bridge.scenes();
    let group_scenes = scenes
        .iter()
        .filter(|s| s.data().group.rid == "690358dc-2a28-4426-9ffe-69567f9dfbf1")
        .take(3)
        .collect::<Vec<_>>();
    let group = group_scenes.get(0).unwrap().group();
    let scene_a = group_scenes.get(0).unwrap();
    let scene_b = group_scenes.get(1).unwrap();
    let scene_c = group_scenes.get(2).unwrap();

    let smart = bridge
        .create_smart_scene(
            SmartScene::builder("My Smart Scene", group)
                .schedule(
                    Schedule::new()
                        // Smart scene is active on weekends
                        .on(&[Weekday::Saturday, Weekday::Sunday])
                        // At 7:00am, activate Scene A
                        .at(TimeslotStart::time(&[07, 00, 00]), scene_a.rid())
                        // At 1:30pm, activate Scene B
                        .at(TimeslotStart::time(&[13, 30, 00]), scene_b.rid())
                        // At sunset, activate Scene C
                        .at(TimeslotStart::Sunset, scene_c.rid()),
                )
                .schedule(
                    Schedule::new()
                        .on(&[Weekday::Monday, Weekday::Wednesday, Weekday::Friday])
                        .at(TimeslotStart::time(&[09, 00, 00]), scene_a.rid()),
                ),
        )
        .await
        .unwrap();

    {
        // Cleanup our test smart scene
        let _ = bridge.delete_smart_scene(smart.id()).await;
    }
}
