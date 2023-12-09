use std::time::Duration;

use hues::{
    api::{Version, V2},
    Bridge, Light,
};

#[tokio::main]
async fn main() {
    // DISCOVERY WITH KEY
    // {
    //     let mut bridge = Bridge::discover()
    //         .await
    //         .unwrap()
    //         .app_key("bbo7vyG7EYdJXIadZaCGtR1SpD969RXs8FohDm1a")
    //         .version(Version::V2)
    //         // .heartbeat(Duration::from_secs(15))
    //         .build();
    // }

    // DISCOVERY CERATE KEY
    // let mut bridge = Bridge::discover()
    //     .await
    //     .unwrap()
    //     .build()
    //     .create_app("magic", "the_gathering")
    //     .await
    //     .unwrap();

    // PRE-EXISTING ADDR AND KEY
    let mut bridge = Bridge::new(
        [10u8, 0, 0, 190],
        "X9UK9xxNDdokkc1pVkqIarALyvjL5vJr8lQMeHs5",
    );

    // let light_ids: Vec<String> = bridge
    //     .lights()
    //     .await
    //     .unwrap()
    //     .keys()
    //     .map(String::to_owned)
    //     .collect();

    // dbg!(&light_ids);

    // let _ = bridge
    //     .command()
    //     .identify(light_ids.iter().nth(0).unwrap())
    //     .identify(light_ids.iter().nth(1).unwrap())
    //     .identify(light_ids.iter().nth(2).unwrap())
    //     .send()
    //     .await;

    for (id, light) in bridge.lights().await.unwrap() {
        let _ = light.command().off().send().await;
        std::thread::sleep(Duration::from_secs(2));
        let _ = light.command().off().send().await;
        std::thread::sleep(Duration::from_secs(2));
    }
}
