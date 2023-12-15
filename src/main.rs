use hues::{
    api::{HueAPIError, HueAPIResponse, Version},
    BasicCommand, Bridge, CIEColor, ColorFeatureBasic, EffectType, GeofenceClientBuilder,
    GeofenceClientCommand, GeolocationCommand, GroupCommand, GroupDimmingState, HomeKitCommand,
    LightAction, LightCommand, MatterCommand, MotionCommand, OnState, ProductArchetype, Resource,
    ResourceIdentifier, ResourceType, SceneAction, SceneBuilder, SceneCommand, SceneEffectState,
    ScenePalette, ScenePaletteColor, SceneStatus, SignalColor, SignalType, Zone, ZoneArchetype,
    ZoneBuilder, ZoneCommand,
};
use rand::prelude::*;
use std::{fmt::Debug, time::Duration};

pub async fn time_async<F, O>(f: F) -> (O, Duration)
where
    F: std::future::Future<Output = O>,
{
    let start = std::time::Instant::now();
    let out = f.await;
    let duration = start.elapsed();
    (out, duration)
}

pub async fn log_time_async<F, O>(f: F) -> O
where
    F: std::future::Future<Output = O>,
    O: Debug,
{
    let (out, duration) = time_async(f).await;
    dbg!(&out, duration);
    out
}

#[tokio::main]
async fn main() {
    let bridge = preexisting_ip_and_key()
        .listen(Duration::from_secs(30))
        .await;

    // log_time_async(bridge.groups());

    log_time_async(alert_lights(&bridge, "#FF0000", "#012345")).await;
    // log_time_async(create_zone(&bridge, "Fun Zone", ZoneArchetype::Computer)).await;
    // log_time_async(change_room_type(&bridge, "Toby's Office", ZoneArchetype::Balcony)).await;
    // log_time_async(rename_room(&bridge, "Toby's Office", "Bat Cave")).await;
    // log_time_async(rename_scene(&bridge)).await;
    // log_time_async(create_scene(&bridge, "TEST SCENE")).await;
    // log_time_async(recall_scene(&bridge, "Diabs")).await;
    // log_time_async(delete_scenes(&bridge, "TEST SCENE")).await;
    // log_time_async(identify_all_lights(&bridge)).await;
    // log_time_async(randomize_all_lights(&bridge)).await;
    // log_time_async(set_specific_light_colors(&bridge, "#FF0000")).await;
}

fn preexisting_ip_and_key() -> Bridge {
    Bridge::new(
        [10u8, 0, 0, 190],
        "X9UK9xxNDdokkc1pVkqIarALyvjL5vJr8lQMeHs5",
    )
}

async fn discover_with_key() -> Bridge {
    Bridge::discover()
        .await
        .unwrap()
        .app_key("bbo7vyG7EYdJXIadZaCGtR1SpD969RXs8FohDm1a")
        .version(Version::V2)
        // .heartbeat(Duration::from_secs(15))
        .build()
}

async fn discover_create_key() -> Bridge {
    let mut bridge = Bridge::discover().await.unwrap().build();
    let _ = &bridge.create_app("magic", "the_gathering").await;
    bridge
}

async fn alert_lights(
    bridge: &Bridge,
    c1: impl Into<String>,
    c2: impl Into<String>,
) -> Result<(), HueAPIError> {
    let _ = bridge
        .group("0d116b60-e996-4eb1-a7af-5a3473a16e27")
        .unwrap()
        .send(&[GroupCommand::Signaling {
            signal: SignalType::Alternating,
            duration: 8000,
            colors: Some(SignalColor::Two(
                CIEColor::from_hex(c1).unwrap(),
                CIEColor::from_hex(c2).unwrap(),
            )),
        }])
        .await?;
    Ok(())
}

// async fn create_zone(
//     bridge: &Bridge,
//     name: impl Into<String>,
//     archetype: ZoneArchetype,
// ) -> Result<(), HueAPIError> {
//     let zone_rid = bridge
//         .create_zone(Zone::builder(name.into(), archetype))
//         .await?;
//     dbg!(zone_rid);
//     Ok(())
// }

// async fn change_room_type(
//     bridge: &Bridge,
//     name: impl Into<String>,
//     archetype: ZoneArchetype,
// ) -> Result<(), HueAPIError> {
//     let name = name.into();
//     let rooms = bridge.rooms().unwrap();
//     for (_id, room) in rooms {
//         if &room.data.metadata.name == &name {
//             let _ = room
//                 .send(&[ZoneCommand::Metadata {
//                     name: None,
//                     archetype: Some(archetype),
//                 }])
//                 .await?;
//         }
//     }
//     Ok(())
// }

// async fn rename_room(
//     bridge: &Bridge,
//     name: impl Into<String>,
//     other_name: impl Into<String>,
// ) -> Result<(), HueAPIError> {
//     let name = name.into();
//     if let Some(tr) = bridge
//         .rooms()
//         .values()
//         .find(|room| &room.data.metadata.name == &name)
//     {
//         dbg!(&tr);
//         let res = tr
//             .send(&[ZoneCommand::Metadata {
//                 name: Some(other_name.into()),
//                 archetype: None,
//             }])
//             .await?;
//         dbg!(res);
//     } else {
//         eprintln!("No scene '{}'", &name);
//     }
//     Ok(())
// }

async fn recall_scene(bridge: &Bridge, name: impl Into<String>) -> Result<(), HueAPIError> {
    let name = name.into();
    if let Some(scene) = bridge.scenes().iter().find(|sc| sc.name() == &name) {
        scene
            .send(&[SceneCommand::Recall {
                action: Some(SceneStatus::DynamicPalette),
                duration: Some(2000),
                dimming: None,
            }])
            .await?;
    } else {
        eprintln!("No scene '{}'", &name);
    }
    Ok(())
}

async fn delete_scenes(bridge: &Bridge, name: impl Into<String>) -> Result<(), HueAPIError> {
    let name = name.into();
    let ids = bridge
        .scenes()
        .iter()
        .filter_map(|sc| {
            if sc.name() == &name {
                Some(sc.id().to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    dbg!(&ids);
    for i in ids {
        bridge.delete_scene(i).await?;
    }
    Ok(())
}

async fn create_scene(bridge: &Bridge, name: impl Into<String>) -> Result<(), HueAPIError> {
    let green = bridge
        .create_scene(
            SceneBuilder::new(
                name.into(),
                ResourceIdentifier {
                    rid: "690358dc-2a28-4426-9ffe-69567f9dfbf1".into(),
                    rtype: ResourceType::Room,
                },
            )
            .actions(vec![
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "509c0477-a1c3-44de-b797-03f528c902b2".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color: Some(ColorFeatureBasic::xy(0.3, 0.4)),
                        ..Default::default()
                    },
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "1e2c0843-88f1-45ad-b182-f78a375e0ea9".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color: Some(ColorFeatureBasic::xy(0.3, 0.4)),
                        ..Default::default()
                    },
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "120b6f18-b912-4211-bb91-038bbfdc918c".into(),
                        rtype: ResourceType::Light,
                    },
                    action: Default::default(),
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "1e84f1c6-71ca-4166-ad3e-034815160617".into(),
                        rtype: ResourceType::Light,
                    },
                    action: Default::default(),
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "7b5d3a37-b09c-4295-9247-3d9d87154118".into(),
                        rtype: ResourceType::Light,
                    },
                    action: Default::default(),
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "c75112fc-6727-46f7-8fe9-cf64021c1a16".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color: Some(ColorFeatureBasic::xy(0.3, 0.4)),
                        ..Default::default()
                    },
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "2aaa6432-d225-4beb-aed7-16bd1fa1d287".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color: Some(ColorFeatureBasic::xy(0.3, 0.4)),
                        ..Default::default()
                    },
                },
            ])
            .palette(ScenePalette {
                color: vec![
                    ScenePaletteColor::xyb(0.1, 0.3, 70.0),
                    ScenePaletteColor::xyb(0.3, 0.5, 70.0),
                ],
                effects: vec![SceneEffectState {
                    effect: Some(EffectType::Candle),
                }],
                ..Default::default()
            }),
        )
        .await?;

    dbg!(green);
    Ok(())
}

// async fn rename_scene(bridge: &Bridge) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
//     let conc2 = bridge
//         .scene("ec1c5855-1c0f-462f-9703-479610af7204")
//         .unwrap();
//     // dbg!(conc2);
//     conc2
//         .send(&[SceneCommand::Metadata {
//             name: Some("Conc2".into()),
//             appdata: None,
//         }])
//         .await
// }

async fn identify_all_lights(bridge: &Bridge) -> Result<(), HueAPIError> {
    for light in bridge.lights() {
        light.identify().await?;
    }
    Ok(())
}

async fn randomize_all_lights(bridge: &Bridge) -> Result<(), HueAPIError> {
    let mut rng = rand::thread_rng();
    loop {
        for light in bridge.lights() {
            if light.data().color.is_some() {
                let _ = light
                    .send(&[
                        LightCommand::Dim(rng.gen_range(70.0..=100.0)),
                        LightCommand::Color {
                            x: rng.gen_range(0.0..=0.5),
                            y: rng.gen_range(0.0..=0.4),
                        },
                    ])
                    .await;
            } else {
                let _ = light
                    .send(&[
                        LightCommand::Dim(80.0),
                        LightCommand::ColorTemp(rng.gen_range(350..=500)),
                    ])
                    .await;
            }
        }
        std::thread::sleep(Duration::from_millis(2000));
    }
}

async fn set_specific_light_colors(
    bridge: &Bridge,
    hex: impl Into<String>,
) -> Result<(), HueAPIError> {
    let mut rng = rand::thread_rng();
    let hex = hex.into();
    for light in bridge.lights() {
        if light.data().color.is_some() {
            let _ = light
                .send(&[
                    LightCommand::color_from_hex(&hex).unwrap(),
                    // LightCommand::color_from_rgb([228, 86, 136]),
                ])
                .await;
        } else {
            let _ = light
                .send(&[
                    LightCommand::Dim(80.0),
                    LightCommand::ColorTemp(rng.gen_range(350..=500)),
                ])
                .await;
        }
    }
    Ok(())
}
