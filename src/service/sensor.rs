use crate::{
    api::HueAPIError,
    command::{
        merge_commands, BasicCommand, GeofenceClientCommand, GeolocationCommand, MotionCommand,
    },
    service::{Bridge, ResourceIdentifier, ResourceType, SetStatus},
};
use serde::{Deserialize, Serialize};

/// A physical contact sensor device.
#[derive(Debug)]
pub struct Contact<'a> {
    bridge: &'a Bridge,
    data: ContactData,
}

impl<'a> Contact<'a> {
    pub fn new(bridge: &'a Bridge, data: ContactData) -> Self {
        Contact { bridge, data }
    }

    pub fn data(&self) -> &ContactData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub async fn send(
        &self,
        commands: &[BasicCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_contact(self.id(), &payload).await
    }
}

/// Internal representation of a [Contact].
#[derive(Clone, Debug, Deserialize)]
pub struct ContactData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Whether sensor is activated or not.
    pub enabled: bool,
    pub contact_report: Option<ContactReport>,
}

impl ContactData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Contact,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContactReport {
    /// Last time the value of this property was updated.
    pub changed: String,
    pub state: ContactStatus,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContactStatus {
    Contact,
    NoContact,
}

/// A motion detection senseor device.
#[derive(Debug)]
pub struct Motion<'a> {
    bridge: &'a Bridge,
    data: MotionData,
}

impl<'a> Motion<'a> {
    pub fn new(bridge: &'a Bridge, data: MotionData) -> Self {
        Motion { bridge, data }
    }

    pub fn data(&self) -> &MotionData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id().to_owned(),
            rtype: ResourceType::Motion,
        }
    }

    pub async fn send(
        &self,
        commands: &[MotionCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_motion(self.id(), &payload).await
    }
}

#[derive(Debug)]
pub struct CameraMotion<'a> {
    bridge: &'a Bridge,
    data: MotionData,
}

/// A camera device with motion detection capability.
impl<'a> CameraMotion<'a> {
    pub fn new(bridge: &'a Bridge, data: MotionData) -> Self {
        CameraMotion { bridge, data }
    }

    pub fn data(&self) -> &MotionData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id().to_owned(),
            rtype: ResourceType::CameraMotion,
        }
    }

    pub async fn send(
        &self,
        commands: &[MotionCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_camera_motion(self.id(), &payload).await
    }
}

/// Internal representation of a [Motion] or [CameraMotion].
#[derive(Clone, Debug, Deserialize)]
pub struct MotionData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Whether sensor is activated or not.
    pub enabled: bool,
    pub motion: MotionState,
    pub sensitivity: Option<Sensitivity>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MotionState {
    /// Motion is valid when `motion_report` property is present, invalid when absent.
    #[deprecated]
    pub motion_valid: bool,
    pub motion_report: Option<MotionReport>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MotionReport {
    /// Last time the value of this property is changed.
    pub changed: String,
    /// `true` if motion is detected/
    pub motion: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Sensitivity {
    pub status: SetStatus,
    /// Sensitivity of the sensor. Value in the range `0` to `sensitivity_max`.
    pub sensitivity: usize,
    /// Maximum value of the sensitivity configuration attribute.
    pub sensitivity_max: Option<usize>,
}

/// A temperature sensor device.
#[derive(Debug)]
pub struct Temperature<'a> {
    bridge: &'a Bridge,
    data: TemperatureData,
}

impl<'a> Temperature<'a> {
    pub fn new(bridge: &'a Bridge, data: TemperatureData) -> Self {
        Temperature { bridge, data }
    }

    pub fn data(&self) -> &TemperatureData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub async fn send(
        &self,
        commands: &[BasicCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_temperature(self.id(), &payload).await
    }
}

/// Internal representation of a [Temperature].
#[derive(Clone, Debug, Deserialize)]
pub struct TemperatureData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Whether sensor is activated or not.
    pub enabled: bool,
    pub temperature: TemperatureState,
}

impl TemperatureData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Temperature,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TemperatureState {
    #[deprecated]
    pub temperature: f32,
    #[deprecated]
    pub temperature_valid: bool,
    pub temperature_report: Option<TemperatureReport>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TemperatureReport {
    /// Last time the value of this property is changed.
    pub changed: String,
    pub temperature: f32,
}

/// A light level detection device.
#[derive(Debug)]
pub struct LightLevel<'a> {
    bridge: &'a Bridge,
    data: LightLevelData,
}

impl<'a> LightLevel<'a> {
    pub fn new(bridge: &'a Bridge, data: LightLevelData) -> Self {
        LightLevel { bridge, data }
    }

    pub fn data(&self) -> &LightLevelData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub async fn send(
        &self,
        commands: &[BasicCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_light_level(self.id(), &payload).await
    }
}

/// Internal representation of a [LightLevel].
#[derive(Clone, Debug, Deserialize)]
pub struct LightLevelData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Whether sensor is activated or not.
    pub enabled: bool,
    pub light: LightLevelState,
}

impl LightLevelData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::LightLevel,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LightLevelState {
    #[deprecated]
    pub light_level: usize,
    #[deprecated]
    pub light_level_valid: bool,
    pub light_level_report: Option<LightLevelReport>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LightLevelReport {
    /// Last time the value of this property is changed.
    pub changed: String,
    /// Light level in `10000*log10(lux) + 1` measured by sensor.
    /// Logarithmic scale used because the human eye adjusts to light levels and small changes at
    /// low lux levels are more noticeable than at high lux levels.
    /// This allows use of linear scale configuration sliders.
    pub light_level: usize,
}

/// A virtual device representing the location of the Hue Bridge.
#[derive(Debug)]
pub struct Geolocation<'a> {
    bridge: &'a Bridge,
    data: GeolocationData,
}

impl<'a> Geolocation<'a> {
    pub fn new(bridge: &'a Bridge, data: GeolocationData) -> Self {
        Geolocation { bridge, data }
    }

    pub fn data(&self) -> &GeolocationData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub async fn send(
        &self,
        commands: &[GeolocationCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_geolocation(self.id(), &payload).await
    }
}

/// Internal representation of the device [Geolocation].
#[derive(Clone, Debug, Deserialize)]
pub struct GeolocationData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Is the geolocation configured.
    pub is_configured: bool,
    /// Info related to today's sun (only available when geolocation has been configured).
    pub sun_today: Option<SunToday>,
}

impl GeolocationData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Geolocation,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SunToday {
    pub sunset_time: String,
    pub day_type: DayType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DayType {
    NormalDay,
    PolarDay,
    PolarNight,
}

/// A virtual device representing a location-based trigger.
#[derive(Debug)]
pub struct GeofenceClient<'a> {
    bridge: &'a Bridge,
    data: GeofenceClientData,
}

impl<'a> GeofenceClient<'a> {
    pub fn new(bridge: &'a Bridge, data: GeofenceClientData) -> Self {
        GeofenceClient { bridge, data }
    }

    pub fn data(&self) -> &GeofenceClientData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn builder(name: impl Into<String>) -> GeofenceClientBuilder {
        GeofenceClientBuilder::new(name)
    }

    pub async fn send(
        &self,
        commands: &[GeofenceClientCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge
            .api
            .put_geofence_client(self.id(), &payload)
            .await
    }
}

/// Internal representation of a [GeofenceClient].
#[derive(Clone, Debug, Deserialize)]
pub struct GeofenceClientData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    pub name: String,
}

impl GeofenceClientData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::GeofenceClient,
        }
    }
}

#[derive(Serialize)]
pub struct GeofenceClientBuilder {
    is_at_home: bool,
    name: String,
}

impl GeofenceClientBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        GeofenceClientBuilder {
            is_at_home: true,
            name: name.into(),
        }
    }

    pub fn is_at_home(mut self, b: bool) -> Self {
        self.is_at_home = b;
        self
    }
}

/// A tamper detection device.
#[derive(Debug)]
pub struct Tamper {
    data: TamperData,
}

impl Tamper {
    pub fn new(data: TamperData) -> Self {
        Tamper { data }
    }

    pub fn data(&self) -> &TamperData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }
}

/// Internal representation of a [Tamper].
#[derive(Clone, Debug, Deserialize)]
pub struct TamperData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    pub tamper_reports: Vec<TamperReport>,
}

impl TamperData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Tamper,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TamperReport {
    /// Last time the value of this property is changed.
    pub changed: String,
    /// Source of tamper and time expired since last change of tamper-state.
    pub source: String,
    /// The state of tamper after last change.
    pub state: TamperStatus,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TamperStatus {
    Tampered,
    NotTampered,
}
