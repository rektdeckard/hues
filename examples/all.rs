use dotenv::dotenv;
use hues::{
    prelude::*,
    service::{
        CIEColor, ColorFeatureBasic, EffectType, LightAction, SceneAction, SceneBuilder,
        SceneColorTempState, SceneEffectState, ScenePalette, ScenePaletteColor, SceneStatus,
        Schedule, SignalType, SmartScene, TimeslotStart, Weekday, Zone, ZoneArchetype,
    },
};
use rand::prelude::*;
use std::{net::IpAddr, time::Duration};

/// NOTE: in order to run examples against a real bridge, you must set the
/// following environment variables to appropriate values:
///
/// HUE_BRIDGE_IP="10.0.0.123"
/// HUE_APP_KEY="abc123xyz789"
/// HUE_CLIENT_KEY="ABC123XYZ789" # only required to use "streaming" features
///
/// You may also set the following to existing resources on your bridge:
#[allow(dead_code)]
const ROOM_NAME: &'static str = "Office"; // The name of a real Room resource
#[allow(dead_code)]
const ROOM_RENAME: &'static str = "Bat Cave";
#[allow(dead_code)]
const ZONE_NAME: &'static str = "Fun Zone"; // The name of a real Zone resource
#[allow(dead_code)]
const SCENE_NAME: &'static str = "Energize"; // The name of a real Scene resource

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

    // let _ = toggle_room(&bridge, ROOM_NAME).await;
    // let _ = smart_scene_stuff(&bridge).await;
    // let _ = alert_lights(&bridge, "#DC00F8", "#0002A3").await;
    // let _ = create_zone(&bridge, ZONE_NAME, ZoneArchetype::Computer).await;
    // let _ = change_room_type(&bridge, ROOM_NAME, ZoneArchetype::Office).await;
    // let _ = rename_room(&bridge, ROOM_NAME, ROOM_RENAME).await;
    // let _ = rename_scene(&bridge).await;
    // let _ = create_scene(&bridge, "TEST SCENE").await;
    // let _ = recall_scene(&bridge, SCENE_NAME).await;
    // let _ = delete_scenes(&bridge, "TEST SCENE").await;
    // let _ = identify_all_lights(&bridge).await;
    // let _ = randomize_all_lights(&bridge).await;
    let _ = set_specific_light_colors(&bridge, "#FF2200").await;
}

#[allow(dead_code)]
async fn toggle_room(bridge: &Bridge, name: &str) -> Result<(), HueAPIError> {
    let room = bridge
        .rooms()
        .into_iter()
        .find(|s| s.name() == name)
        .unwrap();
    room.toggle().await?;
    Ok(())
}

#[allow(dead_code)]
async fn smart_scene_stuff(bridge: &Bridge) -> Result<(), HueAPIError> {
    let _ = bridge
        .delete_smart_scene("becae4e4-faa8-4a39-bc9d-cb16a30241db")
        .await;

    let scenes = bridge.scenes();
    let galaxy = scenes.iter().find(|sc| sc.name() == "Galaxy").unwrap();
    let mf = scenes
        .iter()
        .find(|sc| sc.name() == "Magic Forest")
        .unwrap();
    let diabs = scenes.iter().find(|sc| sc.name() == "Diabs").unwrap();

    let _ss = bridge
        .create_smart_scene(
            SmartScene::builder("I AM SMORT", galaxy.data().group.clone()).schedule(
                Schedule::new()
                    .on(&[Weekday::Saturday, Weekday::Sunday])
                    .at(TimeslotStart::time(&[0, 54, 0]), galaxy.rid())
                    .at(TimeslotStart::time(&[1, 27, 0]), mf.rid())
                    .at(TimeslotStart::time(&[1, 30, 0]), diabs.rid()),
            ),
        )
        .await;

    Ok(())
}

#[allow(dead_code)]
async fn alert_lights(
    bridge: &Bridge,
    c1: impl Into<String>,
    c2: impl Into<String>,
) -> Result<(), HueAPIError> {
    let _ = bridge
        .group("d14bacd9-a352-4f90-912b-6e6f272ff059")
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

#[allow(dead_code)]
async fn create_zone(
    bridge: &Bridge,
    name: impl Into<String>,
    archetype: ZoneArchetype,
) -> Result<(), HueAPIError> {
    let zone = bridge
        .create_zone(Zone::builder(name.into(), archetype))
        .await?;
    dbg!(zone);
    Ok(())
}

#[allow(dead_code)]
async fn change_room_type(
    bridge: &Bridge,
    name: impl Into<String>,
    archetype: ZoneArchetype,
) -> Result<(), HueAPIError> {
    let name = name.into();
    for room in bridge.rooms() {
        if &room.data.metadata.name == &name {
            let _ = room
                .send(&[ZoneCommand::Metadata {
                    name: None,
                    archetype: Some(archetype),
                }])
                .await?;
        }
    }
    Ok(())
}

#[allow(dead_code)]
async fn rename_room(
    bridge: &Bridge,
    name: impl Into<String>,
    other_name: impl Into<String>,
) -> Result<(), HueAPIError> {
    let name = name.into();
    if let Some(tr) = bridge
        .rooms()
        .iter()
        .find(|room| &room.data.metadata.name == &name)
    {
        dbg!(&tr);
        let res = tr
            .send(&[ZoneCommand::Metadata {
                name: Some(other_name.into()),
                archetype: None,
            }])
            .await?;
        dbg!(res);
    } else {
        eprintln!("No room '{}'", &name);
    }
    Ok(())
}

#[allow(dead_code)]
async fn recall_scene(bridge: &Bridge, name: impl Into<String>) -> Result<(), HueAPIError> {
    let name = name.into();
    if let Some(scene) = bridge.scenes().iter().find(|sc| sc.name() == &name) {
        scene
            .send(&[SceneCommand::Recall {
                action: Some(SceneStatus::Active),
                duration: Some(2000),
                dimming: None,
            }])
            .await?;
    } else {
        eprintln!("No scene '{}'", &name);
    }
    Ok(())
}

#[allow(dead_code)]
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

#[allow(dead_code)]
async fn create_scene(bridge: &Bridge, name: impl Into<String>) -> Result<(), HueAPIError> {
    let room = bridge
        .rooms()
        .into_iter()
        .find(|r| r.name() == ROOM_NAME)
        .unwrap();
    let my_scene = bridge
        .create_scene(
            SceneBuilder::new(name.into(), room.rid())
                .actions(
                    room.lights()
                        .into_iter()
                        .map(|light| {
                            let action = if light.supports_color() {
                                LightAction {
                                    color: Some(ColorFeatureBasic::xy(0.3, 0.4)),
                                    ..Default::default()
                                }
                            } else {
                                LightAction {
                                    color_temperature: Some(SceneColorTempState {
                                        mirek: Some(153),
                                    }),
                                    ..Default::default()
                                }
                            };

                            SceneAction {
                                target: light.rid(),
                                action,
                            }
                        })
                        .collect::<Vec<_>>(),
                )
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

    dbg!(my_scene);
    Ok(())
}

#[allow(dead_code)]
async fn rename_scene(bridge: &Bridge) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
    let conc2 = bridge
        .scene("ec1c5855-1c0f-462f-9703-479610af7204")
        .unwrap();
    // dbg!(conc2);
    conc2
        .send(&[SceneCommand::Metadata {
            name: Some("Conc2".into()),
            appdata: None,
        }])
        .await
}

#[allow(dead_code)]
async fn identify_all_lights(bridge: &Bridge) -> Result<(), HueAPIError> {
    for light in bridge.lights() {
        light.identify().await?;
    }
    Ok(())
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
