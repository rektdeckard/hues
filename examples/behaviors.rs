use dotenv::dotenv;
use hues::{
    prelude::*,
    service::{BehaviorInstance, Schedule},
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

    let script = bridge.behavior_scripts().into_iter().nth(2).unwrap();
    dbg!(&script);

    todo!();
    let instance = BehaviorInstance::builder(
        script.id(),
        serde_json::json!({
            "where": [
                { "group": { "rid": "d14bacd9-a352-4f90-912b-6e6f272ff059", "rtype": "room" } }
            ],
            "device": { "rid": bridge.data().unwrap().id, "rtype": ResourceType::Bridge },
        }),
    )
    .name("TEST NIGHTY NIGHT")
    .enabled(true);
    let instance = bridge.create_behavior_instance(instance).await;
    dbg!(&instance);

    if let Ok(instance) = instance {
        // Cleanup our test smart scene
        let _ = bridge.delete_behavior_instance(instance.id()).await;
    }
}
