mod v1;
mod v2;

use crate::device;
use serde::{Deserialize, Serialize};

pub use v2::V2;

#[derive(Debug, Deserialize)]
pub struct HueAPIResponse<D> {
    pub(crate) errors: Vec<HueAPIV2Error>,
    pub(crate) data: D,
}

#[derive(Default, PartialEq)]
pub enum Version {
    V1,
    #[default]
    V2,
}

#[derive(Debug, PartialEq)]
pub enum HueAPIError {
    BadRequest,
    BadDeserialize,
    HueBridgeError(String),
}

#[derive(Debug, Deserialize)]
pub struct HueAPIV2Error {
    /// A human-readable explanation specific to this occurrence of the problem.
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct LightGet {
    /// Type of the supported resource.
    #[serde(rename = "type")]
    pub device_type: device::Device,
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Deprecated: use metadata on device level.
    pub metadata: Metadata,
    pub on: OnState,
    pub dimming: DimmingState,
    pub color_temperature: ColorTempState,
    pub color: Option<ColorState>,
    pub dynamics: DynamicsState,
    pub alert: AlertState,
    /// Feature containing signaling properties.
    pub signaling: SignalingState,
    pub mode: Mode,
    /// Basic feature containing gradient properties.
    pub gradient: Option<GradientState>,
    /// Basic feature containing effect properties.
    pub effects: Option<EffectState>,
    /// Basic feature containing timed effect properties.
    pub timed_effects: Option<TimedEffectState>,
    /// Feature containing properties to configure powerup behaviour of a lightsource.
    pub powerup: Option<PowerupState>,
}

#[derive(Debug, Deserialize)]
pub struct ResourceIdentifier {
    /// The unique id of the referenced resource.
    pub rid: String,
    /// The type of the referenced resource.
    pub rtype: ResourceType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Device,
    BridgeHome,
    Room,
    Zone,
    Light,
    Button,
    RelativeRotary,
    Temperature,
    LightLevel,
    Motion,
    CameraMotion,
    Entertainment,
    Contact,
    Tamper,
    GroupedLight,
    DevicePower,
    ZigbeeBridgeConnectivity,
    ZigbeeConnectivity,
    ZGPConnectivity,
    Bridge,
    ZigbeeDeviceDiscovery,
    #[serde(rename = "homekit")]
    HomeKit,
    Matter,
    MatterFabric,
    Scene,
    EntertainmentConfiguration,
    PublicImage,
    AuthV1,
    BehaviorScript,
    BehaviorInstance,
    Geofence,
    GeofenceClient,
    Geolocation,
    SmartScene,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    /// Human readable name of a resource.
    pub name: String,
    /// Light archetype
    pub archetype: LightArchetype,
    /// A fixed mired value of the white lamp.
    pub fixed_mired: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightArchetype {
    UnknownArchetype,
    ClassicBulb,
    SultanBulb,
    FloodBulb,
    SpotBulb,
    CandleBulb,
    LusterBulb,
    PendantRound,
    PendantLong,
    CeilingRound,
    CeilingSquare,
    FloorShade,
    FloorLantern,
    TableShade,
    RecessedCeiling,
    RecessedFloor,
    SingleSpot,
    DoubleSpot,
    TableWash,
    WallLantern,
    WallShade,
    FlexibleLamp,
    GroundSpot,
    WallSpot,
    Plug,
    HueGo,
    HueLightstrip,
    HueIris,
    HueBloom,
    Bollard,
    WallWasher,
    HuePlay,
    VintageBulb,
    VintageCandleBulb,
    EllipseBulb,
    TriangleBulb,
    SmallGlobeBulb,
    LargeGlobeBulb,
    EdisonBulb,
    ChristmasTree,
    StringLight,
    HueCentris,
    #[serde(rename = "hue_lightstrip_tv")]
    HueLightstripTV,
    #[serde(rename = "hue_lightstrip_pc")]
    HueLightstripPC,
    HueTube,
    HueSigne,
    PendantSpot,
    CeilingHorizontal,
    CeilingTube,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnState {
    /// On/Off state of the light
    ///
    /// on=true
    /// off=false.
    pub on: bool,
}

#[derive(Debug, Deserialize)]
pub struct DimmingState {
    /// Brightness percentage.
    ///
    /// Value cannot be 0, writing 0 changes it to lowest possible brightness.
    pub brightness: f32,
    /// Percentage of the maximum lumen the device outputs on minimum brightness.
    pub min_dim_level: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ColorTempState {
    /// Color temperature in mirek or None when the light color is not in the ct spectrum.
    pub mirek: Option<u16>,
    /// Indication whether the value presented in mirek is valid.
    pub mirek_valid: bool,
    pub mirek_schema: MirekSchema,
}

#[derive(Debug, Deserialize)]
pub struct MirekSchema {
    /// Minimum color temperature this light supports.
    pub mirek_minimum: u16,
    /// Maximum color temperature this light supports.
    pub mirek_maximum: u16,
}

#[derive(Debug, Deserialize)]
pub struct ColorState {
    /// CIE XY gamut position
    pub xy: XYGamut,
    pub gamut: Gamut,
    /// The gammut types supported by hue.
    ///
    /// – A Gamut of early Philips color-only products
    /// – B Limited gamut of first Hue color products
    /// – C Richer color gamut of Hue white and color ambiance products
    /// – Other Color gamut of non-hue products with non-hue gamuts resp w/o gamut
    pub gamut_type: GamutType,
}

/// Color gamut of color bulb.
/// Some bulbs do not properly return the Gamut information. In this case this is not present.
#[derive(Debug, Deserialize)]
pub struct Gamut {
    /// CIE XY gamut position
    pub red: XYGamut,
    /// CIE XY gamut position
    pub green: XYGamut,
    /// CIE XY gamut position
    pub blue: XYGamut,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XYGamut {
    /// X position in color gamut
    pub x: f32,
    /// Y position in color gamut
    pub y: f32,
}

/// The gammut types supported by hue
#[derive(Debug, Deserialize)]
pub enum GamutType {
    /// Gamut of early Philips color-only products
    A,
    /// Limited gamut of first Hue color products
    B,
    /// Richer color gamut of Hue white and color ambiance products
    C,
    /// Color gamut of non-hue products with non-hue gamuts resp w/o gamut
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct DynamicsState {
    /// Current status of the lamp with dynamics.
    pub status: DynamicsStatus,
    /// Statuses in which a lamp could be when playing dynamics.
    pub status_values: Vec<DynamicsStatus>,
    /// Speed of dynamic palette or effect.
    /// The speed is valid for the dynamic palette if the status is [DynamicsStatus::DynamicPalette] or for
    /// the corresponding effect listed in status. In case of status none, the speed is not valid.
    pub speed: f32,
    /// Indicates whether the value presented in speed is valid
    pub speed_valid: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DynamicsStatus {
    DynamicPalette,
    None,
}

#[derive(Debug, Deserialize)]
pub struct AlertState {
    /// Alert effects that the light supports.
    pub action_values: Vec<AlertEffectType>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertEffectType {
    Breathe,
}

#[derive(Debug, Deserialize)]
pub struct SignalingState {
    /// Signals that the light supports.
    pub signal_values: Option<Vec<SignalType>>,
    /// Indicates status of active signal. Not available when inactive.
    pub status: Option<SignalStatus>,
}

#[derive(Debug, Deserialize)]
pub struct SignalStatus {
    /// Indicates which signal is currently active.
    pub signal: SignalType,
    /// Timestamp indicating when the active signal is expected to end. Value is not set if there is NoSignal.
    pub estimated_end: String,
    /// Colors that were provided for the active effect.
    pub colors: Vec<ColorFeatureBasic>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalType {
    /// Stop active signal.
    NoSignal,
    /// Toggle between max brightness and off in fixed color.
    OnOff,
    /// Toggles between off and max brightness with a provided color.
    OnOffColor,
    /// Alternates between two provided colors.
    Alternating,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Normal,
    Streaming,
}

#[derive(Debug, Deserialize)]
pub struct GradientState {
    /// Collection of gradients points.
    /// For control of the gradient points through a PUT a minimum of 2 points need to be provided.
    pub points: Vec<GradientPoint>,
    /// Mode in which the points are currently being deployed.
    /// If not provided during PUT/POST it will be defaulted to InterpolatedPalette.
    pub mode: GradientMode,
    /// Modes a gradient device can deploy the gradient palette of colors.
    pub mode_values: Vec<GradientMode>,
    /// Number of color points that gradient lamp is capable of showing with gradience.
    pub points_capable: usize,
    /// Number of pixels in the device
    pub pixel_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct GradientPoint {
    pub color: ColorFeatureBasic,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorFeatureBasic {
    pub xy: XYGamut,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GradientMode {
    InterpolatedPalette,
    InterpolatedPaletteMirrored,
    RandomPixelated,
}

#[derive(Debug, Deserialize)]
pub struct EffectState {
    pub effect: Option<EffectType>,
    /// Possible effect values you can set in a light.
    pub effect_values: Vec<EffectType>,
    /// Current status values the light is in regarding effects.
    pub status: EffectType,
    /// Possible status values in which a light could be when playing an effect.
    pub status_values: Vec<EffectType>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    Prism,
    Opal,
    Glisten,
    Sparkle,
    Fire,
    Candle,
    NoEffect,
}

#[derive(Debug, Deserialize)]
pub struct TimedEffectState {
    pub effect: Option<TimedEffectType>,
    /// Possible timed effect values you can set in a light.
    pub effect_values: Vec<TimedEffectType>,
    /// Current status values the light is in regarding timed effects.
    pub status: TimedEffectType,
    /// Possible status values in which a light could be when playing a timed effect.
    pub status_values: Vec<TimedEffectType>,
    /// Duration (ms) is mandatory when timed effect is set except for NoEffect.
    /// Resolution decreases for a larger duration. e.g effects with duration smaller than a minute
    /// will be rounded to a resolution of 1s, while effects with duration larger than an hour
    /// will be arounded up to a resolution of 300s. Duration has a max of 21600000 ms.
    pub duration: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimedEffectType {
    Sunrise,
    NoEffect,
}

#[derive(Debug, Deserialize)]
pub struct PowerupState {
    /// When setting the [PowerupPresetType::Custom] preset the additional properties can be set.
    /// For all other presets, no other properties can be included.
    pub preset: PowerupPresetType,
    /// Indicates if the shown values have been configured in the lightsource.
    pub configured: bool,
    /// State to activate after powerup.
    pub on: PowerupOnState,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerupPresetType {
    Safety,
    Powerfail,
    LastOnState,
    Custom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerupOnState {
    /// State to activate after powerup. When setting mode [PowerupOnMode::On], the `on` property must be included.
    pub mode: PowerupOnMode,
    pub on: Option<OnState>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerupOnMode {
    /// Use the value specified in the [PowerupOnState] `on` property.
    On,
    /// Alternate between on and off on each subsequent power toggle.
    Toggle,
    /// Return to the state it was in before powering off.
    Previous,
}
