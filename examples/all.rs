use dotenv::dotenv;
use hues::{
    api::{HueAPIError, HueAPIResponse, Version},
    BasicCommand, Bridge, CIEColor, ColorFeatureBasic, EffectType, GeofenceClientBuilder,
    GeofenceClientCommand, GeolocationCommand, GroupCommand, GroupDimmingState, HomeKitCommand,
    LightAction, LightCommand, MatterCommand, MotionCommand, OnState, ProductArchetype, Resource,
    ResourceIdentifier, ResourceType, SceneAction, SceneBuilder, SceneColorTempState, SceneCommand,
    SceneEffectState, ScenePalette, ScenePaletteColor, SceneStatus, Schedule, SignalColor,
    SignalType, SmartScene, SmartSceneCommand, TimeslotStart, Weekday, Zone, ZoneArchetype,
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
    dotenv().ok();

    let bridge = Bridge::new([10u8, 0, 0, 143], std::env::var("APP_KEY").unwrap())
        .poll(Duration::from_secs(30))
        .await;

    let office = bridge
        .rooms()
        .into_iter()
        .find(|r| r.name() == "Office")
        .unwrap();

    // dbg!(bridge.groups().get(1).unwrap().is_on());

    // log_time_async(smart_scene_stuff(&bridge)).await;
    // log_time_async(alert_lights(&bridge, "#FF1100", "#11FF00")).await;
    // log_time_async(create_zone(&bridge, "Fun Zone", ZoneArchetype::Computer)).await;
    // log_time_async(change_room_type(&bridge, "Office", ZoneArchetype::Office)).await;
    // log_time_async(rename_room(&bridge, "Bat Cave", "Office")).await;
    // log_time_async(rename_scene(&bridge)).await;
    // log_time_async(create_scene(&bridge, "TEST SCENE")).await;
    // log_time_async(recall_scene(&bridge, "Night Work")).await;
    // log_time_async(delete_scenes(&bridge, "TEST SCENE")).await;
    // log_time_async(identify_all_lights(&bridge)).await;
    // log_time_async(randomize_all_lights(&bridge)).await;
    // log_time_async(set_specific_light_colors(&bridge, "#FF2200")).await;
}

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

    // let cmd = SmartSceneCommand::create_schedule()
    //     .on(&[Weekday::Saturday, Weekday::Sunday])
    //     .at(TimeslotStart::time(&[0, 20, 0]), tokyo.rid())
    //     .build();

    let ss = bridge
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

    // dbg!(ss);

    Ok(())
}

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
                    rid: "eab284ce-48ad-4f9f-9d21-4da1af36099d".into(),
                    rtype: ResourceType::Room,
                },
            )
            .actions(vec![
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "1ea8644a-db7b-4f4a-a4b6-87703a296cce".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color: Some(ColorFeatureBasic::xy(0.3, 0.4)),
                        ..Default::default()
                    },
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "909c4644-122e-4a52-b687-370bef85eb72".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color: Some(ColorFeatureBasic::xy(0.3, 0.4)),
                        ..Default::default()
                    },
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "fd6e67f8-da20-4fc1-81ae-459e039abfe9".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color: Some(ColorFeatureBasic::xy(0.3, 0.4)),
                        ..Default::default()
                    },
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "61025bd7-6150-4b7f-b1ea-70b4c3dba5c4".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color: Some(ColorFeatureBasic::xy(0.3, 0.4)),
                        ..Default::default()
                    },
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "7f1f28a2-be41-49bd-a27f-28e36980a05a".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color: Some(ColorFeatureBasic::xy(0.3, 0.4)),
                        ..Default::default()
                    },
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "96904b1a-4024-48d7-9d40-f6c985bf18a4".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color_temperature: Some(SceneColorTempState { mirek: Some(153) }),
                        ..Default::default()
                    },
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "d449485b-3244-452c-ac23-43586b7253cf".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color_temperature: Some(SceneColorTempState { mirek: Some(153) }),
                        ..Default::default()
                    },
                },
                SceneAction {
                    target: ResourceIdentifier {
                        rid: "8b5a45e8-bdba-4263-95e7-7042129d7542".into(),
                        rtype: ResourceType::Light,
                    },
                    action: LightAction {
                        color_temperature: Some(SceneColorTempState { mirek: Some(153) }),
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
