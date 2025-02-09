use dotenv::dotenv;
use hues::{
    command::EntertainmentConfigurationCommand,
    service::{Bridge, ResourceType},
};
use std::{net::IpAddr, time::Duration};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bridge = Bridge::new_streaming(
        std::env::var("HUE_BRIDGE_IP")
            .unwrap()
            .parse::<IpAddr>()
            .unwrap(),
        std::env::var("HUE_APP_KEY").unwrap(),
        std::env::var("HUE_CLIENT_KEY").unwrap(),
    );
    let bridge = bridge
        .listen(|changes| {
            for ri in &changes {
                match ri.rtype {
                    ResourceType::EntertainmentConfiguration => {
                        dbg!(&ri);
                    }
                    _ => {}
                }
            }
        })
        .await;

    let ents = bridge.entertainment_configurations();
    let ent = ents.get(0).unwrap();
    // dbg!(
    //     ent.send(&[EntertainmentConfigurationCommand::Action(
    //         hues::EntertainmentAction::Start,
    //     )])
    //     .await
    // );
    // dbg!(ent.open_stream().await);

    dbg!(bridge.initialize_streaming(ent.id()).await);

    loop {}
}
