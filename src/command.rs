use json_patch::merge;
use serde::{ser::SerializeMap, Serialize};
use serde_json::json;

use crate::{
    api::{
        AlertEffectType, ColorFeatureBasic, EffectType, GradientMode, HueAPIError, OnState,
        PowerupOnState, PowerupPresetType, SignalType, TimedEffectType, XYGamut,
    },
    Bridge,
};

pub struct Command;

impl Command {
    pub fn merge<S: Serialize>(commands: &[S]) -> serde_json::Value {
        let mut map = json!({});
        for cmd in commands {
            merge(&mut map, &serde_json::to_value(cmd).unwrap());
        }

        map
    }
}

pub struct CommandBuilder<'b> {
    bridge: &'b Bridge<'b>,
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

    pub async fn send(&self) -> Result<(), HueAPIError> {
        for cmd in &self.commands {
            match cmd {
                CommandType::Light(id, lc) => match lc {
                    LightCommand::Identify => {
                        self.bridge.api.identify_light(id).await?;
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }

        Ok(())
    }
}

pub enum CommandType {
    BehaviorInstance(BehaviorInstanceCommand),
    Bridge(BridgeCommand),
    Button(ButtonCommand),
    CameraMotion(CameraMotionCommand),
    Contact(ContactCommand),
    Device(DeviceCommand),
    DevicePower(DevicePowerCommand),
    Entertainment(EntertainmentCommand),
    EntertainmentConfiguration(EntertainmentConfigurationCommand),
    GeofenceClient(GeofenceClientCommand),
    Geolocation(GeolocationCommand),
    GroupedLight(GroupedLightCommand),
    HomeKit(HomeKitCommand),
    Light(String, LightCommand),
    LightLevel(String, LightLevelCommand),
    Matter(MatterCommand),
    MatterFabric(MatterFabricCommand),
    Motion(MotionCommand),
    RelativeRotary(RelativeRotaryCommand),
    Room(RoomCommand),
    Scene(SceneCommand),
    SmartScene(SmartSceneCommand),
    Tamper(TamperCommand),
    Temperature(TemperatureCommand),
    ZGPConnectivity(ZGPConnectivityCommand),
    ZigbeeConnectivity(ZigbeeConnectivityCommand),
    ZigbeeDeviceDiscovery(ZigbeeDeviceDiscoveryCommand),
    Zone(ZoneCommand),
}

pub struct BehaviorInstanceCommand;

pub struct BridgeCommand;

pub struct ButtonCommand;

pub struct CameraMotionCommand;

pub struct ContactCommand;

pub struct DeviceCommand;

pub struct DevicePowerCommand;

pub struct EntertainmentCommand;

pub struct EntertainmentConfigurationCommand;

pub struct GeofenceClientCommand;

pub struct GeolocationCommand;

pub struct GroupedLightCommand;

pub struct HomeKitCommand;

pub enum LightCommand {
    Alert(AlertEffectType),
    /// CIE XY gamut position
    Color {
        /// X position in color gamut (0-1)
        x: f32,
        /// Y position in color gamut (0-1)
        y: f32,
    },
    /// Color temperature in mirek (153-500) or null when the light color is not in the ct spectrum.
    ColorTemp(i32),
    ColorTempDelta {
        action: DeltaAction,
        /// Mirek delta to current mirek (0-347). Clip at mirek_minimum and mirek_maximum of mirek_schema.
        mirek_delta: Option<i32>,
    },
    /// Brightness percentage. value cannot be 0, writing 0 changes it to lowest possible brightness.
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
        /// The speed is valid for the dynamic palette if the status is [DynamicsStatus::DynamicPalette](crate::api::DynamicsStatus::DynamicPalette)
        /// or for the corresponding effect listed in status. In case of status [None], the speed is not valid.
        speed: Option<f32>,
    },
    /// Basic feature containing gradient properties.
    Gradient {
        /// Collection of gradients points. For control of the gradient points through a PUT a minimum of 2 points need to be provided.
        points: Vec<XYGamut>,
        mode: Option<GradientMode>,
    },
    /// Basic feature containing effect properties.
    Effect(EffectType),
    /// Triggers a visual identification sequence, performing one breathe cycle.
    Identify,
    Power(bool),
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
        /// Duration has a max of 65534000 ms and a stepsize of 1 second.
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

#[derive(Debug)]
pub struct PowerupColor {
    /// State to activate after powerup.
    ///
    /// Availability of [PowerupColorMode::ColorTemp] and [PowerupColorMode::Color] modes depend on
    /// the capabilities of the lamp.
    mode: PowerupColorMode,
    color: Option<XYGamut>,
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
    One(XYGamut),
    Two(XYGamut, XYGamut),
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
            Self::Power(on) => {
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

pub struct LightLevelCommand;

pub struct MatterCommand;

pub struct MatterFabricCommand;

pub struct MotionCommand;

pub struct RelativeRotaryCommand;

pub struct RoomCommand;

pub struct SceneCommand;

pub struct SmartSceneCommand;

pub struct TamperCommand;

pub struct TemperatureCommand;

pub struct ZGPConnectivityCommand;

pub struct ZigbeeConnectivityCommand;

pub struct ZigbeeDeviceDiscoveryCommand;

pub struct ZoneCommand;
