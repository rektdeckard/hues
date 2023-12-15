use hues::{
    api::{HueAPIError, HueAPIResponse, Version},
    BasicCommand, Bridge, CIEColor, ColorFeatureBasic, EffectType, GeofenceClientBuilder,
    GeofenceClientCommand, GeolocationCommand, GroupCommand, GroupDimmingState, HomeKitCommand,
    LightAction, LightCommand, MatterCommand, MotionCommand, OnState, ProductArchetype, Resource,
    ResourceIdentifier, ResourceType, SceneAction, SceneBuilder, SceneCommand, SceneEffectState,
    ScenePalette, ScenePaletteColor, SceneStatus, SignalColor, SignalType, Zone, ZoneArchetype,
    ZoneBuilder, ZoneCommand,
};

#[tokio::main]
async fn main() {}

async fn discover_with_key(app_key: impl Into<String>) -> Bridge {
    Bridge::discover()
        .await
        .unwrap()
        .app_key(&app_key.into())
        .version(Version::V2)
        .build()
}

async fn discover_create_key(
    app_name: impl Into<String>,
    instance_name: impl Into<String>,
) -> Bridge {
    let mut bridge = Bridge::discover().await.unwrap().build();
    let _ = &bridge.create_app(app_name, instance_name).await;
    bridge
}
