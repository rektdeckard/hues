use hues::{
    api::{SignalType, Version, XYGamut},
    Bridge, LightCommand, SignalColor,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut bridge = preexisting_ip_and_key().await;

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

    for (_id, light) in bridge.lights().await.unwrap() {
        // if light.data.color.is_some() {
        // let _ = light
        //     .send(&[LightCommand::PowerUp {
        //         preset: hues::api::PowerupPresetType::LastOnState,
        //         on: None,
        //         dimming: None,
        //         color: None,
        //     }])
        //     .await;
        // } else {
        // }
        // std::thread::sleep(Duration::from_secs(1));
        let r = light
            .send(&[LightCommand::Effect(hues::api::EffectType::Candle)])
            .await;
        dbg!(r);
    }
}

async fn preexisting_ip_and_key<'a>() -> Bridge<'a> {
    Bridge::new(
        [10u8, 0, 0, 190],
        "X9UK9xxNDdokkc1pVkqIarALyvjL5vJr8lQMeHs5",
    )
}

async fn discover_with_key<'a>() -> Bridge<'a> {
    Bridge::discover()
        .await
        .unwrap()
        .app_key("bbo7vyG7EYdJXIadZaCGtR1SpD969RXs8FohDm1a")
        .version(Version::V2)
        // .heartbeat(Duration::from_secs(15))
        .build()
}

async fn discover_create_key<'a>() -> Bridge<'a> {
    let mut bridge = Bridge::discover().await.unwrap().build();
    let _ = &bridge.create_app("magic", "the_gathering").await;
    bridge
}
