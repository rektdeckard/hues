use crate::{
    api::HueAPIError,
    service::{
        bridge::Bridge,
        device::ProductArchetype,
        group::GroupDimmingState,
        light::{
            AlertEffectType, CIEColor, ColorFeatureBasic, EffectType, GradientMode, OnState,
            ParseColorError, PowerupOnState, PowerupPresetType, SignalType, TimedEffectType,
        },
        resource::ResourceIdentifier,
        scene::{SceneAction, ScenePalette, SceneStatus, Schedule},
        zigbee::ZigbeeChannel,
        zone::ZoneArchetype,
    },
};
use json_patch::merge;
use serde::{ser::SerializeMap, Serialize};
use serde_json::json;

pub fn merge_commands<S: Serialize>(commands: &[S]) -> serde_json::Value {
    let mut map = json!({});
    for cmd in commands {
        merge(&mut map, &serde_json::to_value(cmd).unwrap());
    }
    map
}

pub struct CommandBuilder<'b> {
    bridge: &'b Bridge,
    commands: Vec<CommandType>,
}

impl<'b> CommandBuilder<'b> {
    pub fn new(bridge: &'b Bridge) -> Self {
        CommandBuilder {
            bridge,
            commands: vec![],
        }
    }

    pub fn identify(mut self, id: impl Into<String>) -> Self {
        self.commands
            .push(CommandType::Light(id.into(), LightCommand::Identify));
        self
    }

    pub async fn send(&self) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let mut changes = vec![];

        for cmd in &self.commands {
            match cmd {
                CommandType::Light(id, lc) => match lc {
                    LightCommand::Identify => {
                        todo!();
                        // let res = self
                        //     .bridge
                        //     .api
                        //     .put_light(id, &serde_json::to_value(lc).unwrap())
                        //     .await?;
                        // changes.extend(res);
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }

        Ok(changes)
    }
}

pub enum CommandType {
    BehaviorInstance(BehaviorInstanceCommand),
    Bridge(BridgeCommand),
    Button(ButtonCommand),
    CameraMotion(CameraMotionCommand),
    Contact(BasicCommand),
    Device(DeviceCommand),
    DevicePower(DevicePowerCommand),
    EntertainmentConfiguration(EntertainmentConfigurationCommand),
    GeofenceClient(GeofenceClientCommand),
    Geolocation(GeolocationCommand),
    GroupedLight(GroupCommand),
    HomeKit(HomeKitCommand),
    Light(String, LightCommand),
    LightLevel(BasicCommand),
    Matter(MatterCommand),
    MatterFabric(MatterFabricCommand),
    Motion(MotionCommand),
    RelativeRotary(RelativeRotaryCommand),
    Room(ZoneCommand),
    Scene(SceneCommand),
    SmartScene(SmartSceneCommand),
    Tamper(TamperCommand),
    Temperature(BasicCommand),
    ZigbeeConnectivity(ZigbeeConnectivityCommand),
    ZigbeeDeviceDiscovery(ZigbeeDeviceDiscoveryCommand),
    Zone(ZoneCommand),
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BasicCommand {
    Enabled(bool),
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorInstanceCommand {
    /// Indicated whether a scripts is enabled.
    Enabled(bool),
    /// Script configuration.
    /// This property is validated using ScriptDefinition.configuration_schema JSON schema.
    Configuration(serde_json::Value),
    /// Action that needs to be taken by this script instance.
    /// This property is validated using ScriptDefinition.trigger_schema JSON schema.
    Trigger(serde_json::Value),
    Metadata {
        name: String,
    },
}

pub struct BridgeCommand;

pub struct ButtonCommand;

pub struct CameraMotionCommand;

pub enum DeviceCommand {
    Identify,
    Metadata {
        name: Option<String>,
        archetype: Option<ProductArchetype>,
    },
    /// Activates or extends user usertest mode of device for 120 seconds.
    /// `false` deactivates usertest mode. In usertest mode, devices report
    /// changes in state faster and indicate state changes on device LED.
    UserTest(bool),
}

impl Serialize for DeviceCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Identify => {
                map.serialize_entry("identify", &json!({ "action": "identify" }))?;
            }
            Self::Metadata { name, archetype } => {
                map.serialize_entry("metadata", &json!({ "name": name, "archetype": archetype }))?;
            }
            Self::UserTest(u) => {
                map.serialize_entry("usertest", &json!({ "usertest": u }))?;
            }
        }
        map.end()
    }
}

pub struct DevicePowerCommand;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntertainmentConfigurationCommand {
    Action(EntertainmentAction),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntertainmentAction {
    Start,
    Stop,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GeofenceClientCommand {
    IsAtHome(bool),
    Name(String),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum GeolocationCommand {
    Coordinates {
        #[serde(rename = "latitude")]
        lat: f32,
        #[serde(rename = "longitude")]
        lon: f32,
    },
}

pub enum GroupCommand {
    Alert(AlertEffectType),
    /// CIE XY gamut position
    Color {
        /// X position in color gamut (`0`-`1`)
        x: f32,
        /// Y position in color gamut (`0`-`1`)
        y: f32,
    },
    /// Color temperature in mirek \[`153`-`500`\] or null when the light color is not in the ct spectrum.
    ColorTemp(u16),
    /// Brightness percentage \[`0.0`-`100.0`\]. value cannot be `0`, writing `0` changes it to lowest possible brightness.
    ColorTempDelta {
        action: DeltaAction,
        /// Mirek delta to current mirek \[`0`-`347`\]. Clip at mirek_minimum and mirek_maximum of mirek_schema.
        mirek_delta: Option<u16>,
    },
    Dim(f32),
    DimDelta {
        action: DeltaAction,
        /// Brightness percentage of full-scale increase delta to current dimlevel. Clip at Max-level or Min-level.
        brightness_delta: Option<f32>,
    },
    Dynamics {
        /// Duration of a light transition or timed effects in ms.
        duration: Option<usize>,
    },
    On(bool),
    /// Feature containing signaling properties.
    Signaling {
        signal: SignalType,
        /// Duration in seconds.
        ///
        /// Has a max of 65534000 ms and a stepsize of 1 second.
        /// Values in between steps will be rounded. Duration is ignored for [SignalType::NoSignal].
        duration: usize,
        /// List of colors (1 or 2) to apply to the signal (not supported by all signals).
        colors: Option<SignalColor>,
    },
}

impl Serialize for GroupCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Alert(effect) => {
                map.serialize_entry("alert", &json!({ "action": effect }))?;
            }
            Self::Color { x, y } => {
                map.serialize_entry("color", &json!({ "xy": { "x": x, "y": y } }))?;
            }
            Self::ColorTemp(mirek) => {
                map.serialize_entry("color_temperature", &json!({ "mirek": mirek }))?;
            }
            Self::ColorTempDelta {
                action,
                mirek_delta,
            } => {
                map.serialize_entry(
                    "color_temperature_delta",
                    &json!({ "action": action, "mirek_delta": mirek_delta}),
                )?;
            }
            Self::Dim(pct) => {
                map.serialize_entry("dimming", &json!({ "brightness": pct }))?;
            }
            Self::DimDelta {
                action,
                brightness_delta,
            } => {
                map.serialize_entry(
                    "dimming_delta",
                    &json!({ "action": action, "brightness_delta": brightness_delta }),
                )?;
            }
            Self::Dynamics { duration } => {
                map.serialize_entry("dynamics", &json!({ "duration": duration }))?;
            }
            Self::Signaling {
                signal,
                duration,
                colors,
            } => {
                map.serialize_entry(
                    "signaling",
                    &json!({
                        "signal": signal,
                        "duration": duration,
                        "colors": colors,
                    }),
                )?;
            }
            Self::On(on) => {
                map.serialize_entry("on", &OnState { on: *on })?;
            }
        }
        map.end()
    }
}

#[derive(Debug)]
pub enum HomeKitCommand {
    Reset,
}

impl Serialize for HomeKitCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Reset => map.serialize_entry("action", "homekit_reset")?,
        }
        map.end()
    }
}

pub enum LightCommand {
    Alert(AlertEffectType),
    /// CIE XY gamut position
    Color {
        /// X position in color gamut (`0`-`1`)
        x: f32,
        /// Y position in color gamut (`0`-`1`)
        y: f32,
    },
    /// Color temperature in mirek \[`153`-`500`\] or null when the light color is not in the ct spectrum.
    ColorTemp(u16),
    ColorTempDelta {
        action: DeltaAction,
        /// Mirek delta to current mirek \[`0`-`347`\]. Clip at mirek_minimum and mirek_maximum of mirek_schema.
        mirek_delta: Option<u16>,
    },
    /// Brightness percentage \[`0.0`-`100.0`\]. value cannot be `0`, writing `0` changes it to lowest possible brightness.
    Dim(f32),
    DimDelta {
        action: Option<DeltaAction>,
        /// Brightness percentage of full-scale increase delta to current dimlevel. Clip at Max-level or Min-level.
        brightness_delta: Option<f32>,
    },
    Dynamics {
        /// Duration of a light transition or timed effects in ms.
        duration: Option<usize>,
        /// Speed of dynamic palette or effect.
        ///
        /// The speed is valid for the dynamic palette if the status is [DynamicsStatus::DynamicPalette](crate::service::light::DynamicsStatus::DynamicPalette)
        /// or for the corresponding effect listed in status. In case of status [None], the speed is not valid.
        speed: Option<f32>,
    },
    /// Basic feature containing gradient properties.
    Gradient {
        /// Collection of gradients points. For control of the gradient points through a PUT a minimum of 2 points need to be provided.
        points: Vec<CIEColor>,
        mode: Option<GradientMode>,
    },
    /// Basic feature containing effect properties.
    Effect(EffectType),
    /// Triggers a visual identification sequence, performing one breathe cycle.
    Identify,
    Metadata {
        name: Option<String>,
        archetype: Option<ProductArchetype>,
    },
    On(bool),
    PowerUp {
        /// When setting the custom preset the additional properties can be set.
        /// For all other presets, no other properties can be included.
        preset: PowerupPresetType,
        on: Option<PowerupOnState>,
        dimming: Option<PowerupDimming>,
        color: Option<PowerupColor>,
    },
    /// Feature containing signaling properties.
    Signaling {
        signal: SignalType,
        /// Duration in seconds.
        ///
        /// Has a max of 65534000 ms and a stepsize of 1 second.
        /// Values in between steps will be rounded. Duration is ignored for [SignalType::NoSignal].
        duration: usize,
        /// List of colors (1 or 2) to apply to the signal (not supported by all signals).
        colors: Option<SignalColor>,
    },
    /// Basic feature containing timed effect properties.
    TimedEffect {
        effect: TimedEffectType,
        /// Duration is mandatory when timed effect is set, except for [TimedEffectType::NoEffect].
        /// Resolution decreases for a larger duration. e.g Effects with duration smaller than
        /// a minute will be rounded to a resolution of 1s, while effects with duration larger than
        /// an hour will be arounded up to a resolution of 300s. Duration has a max of 21600000 ms.
        duration: Option<usize>,
    },
}

impl LightCommand {
    pub fn color_from_rgb(rgb: [u8; 3]) -> LightCommand {
        let cie = CIEColor::from_rgb(rgb);
        LightCommand::Color { x: cie.x, y: cie.y }
    }

    pub fn color_from_hex(hex: impl Into<String>) -> Result<LightCommand, ParseColorError> {
        match CIEColor::from_hex(hex) {
            Ok(cie) => Ok(LightCommand::Color { x: cie.x, y: cie.y }),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub struct PowerupColor {
    /// State to activate after powerup.
    ///
    /// Availability of [PowerupColorMode::ColorTemp] and [PowerupColorMode::Color] modes depend on
    /// the capabilities of the lamp.
    mode: PowerupColorMode,
    color: Option<CIEColor>,
    color_temperature: Option<u16>,
}

impl Serialize for PowerupColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("mode", &self.mode)?;
        if let Some(xy) = &self.color {
            map.serialize_entry("color", &ColorFeatureBasic { xy: xy.clone() })?;
        }
        if let Some(temp) = self.color_temperature {
            map.serialize_entry("color_temperature", &json!({ "mirek": temp }))?;
        }
        map.end()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerupColorMode {
    Color,
    #[serde(rename = "color_temperature")]
    ColorTemp,
    Previous,
}

#[derive(Debug)]
pub struct PowerupDimming {
    mode: PowerupDimmingMode,
    brightness: Option<f32>,
}

impl Serialize for PowerupDimming {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("mode", &self.mode)?;
        if let Some(bri) = self.brightness {
            map.serialize_entry("dimming", &json!({ "brightness": bri }))?;
        }
        map.end()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerupDimmingMode {
    Dimming,
    Previous,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeltaAction {
    Up,
    Down,
    Stop,
}

#[derive(Debug)]
pub enum SignalColor {
    One(CIEColor),
    Two(CIEColor, CIEColor),
}

impl Serialize for SignalColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SignalColor::One(inner) => {
                serializer.collect_seq([ColorFeatureBasic { xy: inner.clone() }])
            }
            SignalColor::Two(inner_a, inner_b) => serializer.collect_seq([
                ColorFeatureBasic {
                    xy: inner_a.clone(),
                },
                ColorFeatureBasic {
                    xy: inner_b.clone(),
                },
            ]),
        }
    }
}

impl Serialize for LightCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Alert(effect) => {
                map.serialize_entry("alert", &json!({ "action": effect }))?;
            }
            Self::Color { x, y } => {
                map.serialize_entry("color", &json!({ "xy": { "x": x, "y": y } }))?;
            }
            Self::ColorTemp(mirek) => {
                map.serialize_entry("color_temperature", &json!({ "mirek": mirek }))?;
            }
            Self::ColorTempDelta {
                action,
                mirek_delta,
            } => {
                map.serialize_entry(
                    "color_temperature_delta",
                    &json!({ "action": action, "mirek_delta": mirek_delta}),
                )?;
            }
            Self::Dim(pct) => {
                map.serialize_entry("dimming", &json!({ "brightness": pct }))?;
            }
            Self::DimDelta {
                action,
                brightness_delta,
            } => {
                map.serialize_entry(
                    "dimming_delta",
                    &json!({ "action": action, "brightness_delta": brightness_delta }),
                )?;
            }
            Self::Dynamics { duration, speed } => {
                map.serialize_entry("dynamics", &json!({ "duration": duration, "speed": speed }))?;
            }
            Self::Effect(effect) => {
                map.serialize_entry("effects", &json!({ "effect": effect }))?;
            }
            Self::Gradient { points, mode } => {
                let points = points
                    .iter()
                    .map(|xy| ColorFeatureBasic { xy: xy.clone() })
                    .collect::<Vec<ColorFeatureBasic>>();
                map.serialize_entry("gradient", &json!({ "points": points, "mode": mode }))?;
            }
            Self::Identify => {
                map.serialize_entry("identify", &json!({ "action": "identify" }))?;
            }
            Self::Metadata { name, archetype } => {
                map.serialize_entry("metadata", &json!({ "name": name, "archetype": archetype }))?;
            }
            Self::On(on) => {
                map.serialize_entry("on", &OnState { on: *on })?;
            }
            Self::PowerUp {
                preset,
                on,
                dimming,
                color,
            } => {
                map.serialize_entry(
                    "powerup",
                    &json!({
                        "preset": preset,
                        "on": on,
                        "dimming": dimming,
                        "color": color
                    }),
                )?;
            }
            Self::Signaling {
                signal,
                duration,
                colors,
            } => {
                map.serialize_entry(
                    "signaling",
                    &json!({
                        "signal": signal,
                        "duration": duration,
                        "colors": colors,
                    }),
                )?;
            }
            Self::TimedEffect { effect, duration } => {
                map.serialize_entry(
                    "timed_effects",
                    &json!({ "effect": effect, "duration": duration }),
                )?;
            }
        }
        map.end()
    }
}

#[derive(Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceIdentifyType {
    /// Performs Zigbee LED identification cycles for 5 seconds.
    Bridge,
    /// Perform one breathe cycle.
    #[default]
    Lights,
    /// Perform LED identification cycles for 15 seconds.
    Sensors,
}

#[derive(Debug)]
pub enum MatterCommand {
    Reset,
}

impl Serialize for MatterCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Reset => map.serialize_entry("action", "matter_reset")?,
        }
        map.end()
    }
}

pub struct MatterFabricCommand;

pub enum MotionCommand {
    Enabled(bool),
    Sensitivity(usize),
}

impl Serialize for MotionCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Enabled(e) => {
                map.serialize_entry("enabled", e)?;
            }
            Self::Sensitivity(s) => {
                map.serialize_entry("sensitivity", &json!({ "sensitivity": s }))?;
            }
        }
        map.end()
    }
}

pub struct RelativeRotaryCommand;

pub enum ZoneCommand {
    Children(Vec<ResourceIdentifier>),
    Metadata {
        name: Option<String>,
        archetype: Option<ZoneArchetype>,
    },
}

impl Serialize for ZoneCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Children(rids) => {
                map.serialize_entry("children", rids)?;
            }
            Self::Metadata { name, archetype } => {
                map.serialize_entry("metadata", &json!({ "name": name, "archetype": archetype }))?;
            }
        }
        map.end()
    }
}

pub enum SceneCommand {
    /// List of actions to be executed synchronously on recall.
    Actions(Vec<SceneAction>),
    /// Indicates whether to automatically start the scene dynamically on [SceneStatus::Active] recall.
    AutoDynamic(bool),
    Metadata {
        name: Option<String>,
        appdata: Option<String>,
    },
    /// Group of colors that describe the palette of colors to be used when playing dynamics.
    Palette(ScenePalette),
    /// Trigger the scene, optionally overriding some of its properties.
    Recall {
        /// When writing [SceneStatus::Active], the actions in the scene are executed on the target.
        /// [SceneStatus::DynamicPalette] starts dynamic scene with colors in the Palette object.
        action: Option<SceneStatus>,
        /// Transition to the scene within the timeframe given by duration in ms.
        duration: Option<usize>,
        /// Override the scene dimming/brightness.
        dimming: Option<GroupDimmingState>,
    },
    /// Speed of dynamic palette for this scene.
    Speed(f32),
}

impl Serialize for SceneCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Actions(actions) => {
                map.serialize_entry("actions", actions)?;
            }
            Self::AutoDynamic(a) => {
                map.serialize_entry("auto_dynamic", a)?;
            }
            Self::Metadata { name, appdata } => {
                map.serialize_entry("metadata", &json!({ "name": name, "appdata": appdata}))?;
            }
            Self::Palette(palette) => {
                map.serialize_entry("palette", palette)?;
            }
            Self::Recall {
                action,
                duration,
                dimming,
            } => {
                map.serialize_entry(
                    "recall",
                    &json!({ "action": action, "duration": duration, "dimming": dimming }),
                )?;
            }
            Self::Speed(s) => {
                map.serialize_entry("speed", s)?;
            }
        }
        map.end()
    }
}

pub enum SmartSceneCommand {
    On(bool),
    Metadata {
        name: Option<String>,
        appdata: Option<String>,
    },
    Schedule(Vec<Schedule>),
    TransitionDuration(usize),
}

impl SmartSceneCommand {
    pub fn create_schedule() -> Schedule {
        Schedule::new()
    }
}

impl Serialize for SmartSceneCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::On(b) => {
                map.serialize_entry(
                    "recall",
                    &json!({ "action": if *b { "activate" } else { "deactivate"} }),
                )?;
            }
            Self::Metadata { name, appdata } => {
                map.serialize_entry("metadata", &json!({ "name": name, "appdata": appdata}))?;
            }
            Self::Schedule(ts) => {
                map.serialize_entry("week_timeslots", ts)?;
            }
            Self::TransitionDuration(ms) => {
                map.serialize_entry("transition_duration", ms)?;
            }
        }
        map.end()
    }
}

pub struct TamperCommand;

pub enum ZigbeeConnectivityCommand {
    Channel(ZigbeeChannel),
}

impl Serialize for ZigbeeConnectivityCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Channel(ch) => {
                map.serialize_entry("channel", &json!({ "value": ch }))?;
            }
        }
        map.end()
    }
}

pub enum ZigbeeDeviceDiscoveryCommand {
    Action {
        search_codes: Vec<String>,
        install_codes: Vec<String>,
    },
}

impl Serialize for ZigbeeDeviceDiscoveryCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!();
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Action {
                search_codes,
                install_codes,
            } => {
                map.serialize_entry(
                    "action",
                    &json!({
                        "action_type": "search",
                        "search_codes": search_codes,
                        "install_codes": install_codes
                    }),
                )?;
            }
        }
        map.end()
    }
}
