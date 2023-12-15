use super::{
    device::SetStatus,
    resource::{ResourceIdentifier, ResourceType},
};
use crate::{
    api::{BridgeClient, HueAPIError},
    command::{
        merge_commands, BasicCommand, GeofenceClientCommand, GeolocationCommand, MotionCommand,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Motion<'a> {
    api: &'a BridgeClient,
    data: MotionData,
}

impl<'a> Motion<'a> {
    pub fn new(api: &'a BridgeClient, data: MotionData) -> Self {
        Motion { api, data }
    }

    pub fn data(&self) -> &MotionData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub async fn send(
        &self,
        commands: &[MotionCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_motion(self.id(), &payload).await
    }
}

#[derive(Debug)]
pub struct CameraMotion<'a> {
    api: &'a BridgeClient,
    data: MotionData,
}

impl<'a> CameraMotion<'a> {
    pub fn new(api: &'a BridgeClient, data: MotionData) -> Self {
        CameraMotion { api, data }
    }

    pub fn data(&self) -> &MotionData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub async fn send(
        &self,
        commands: &[MotionCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_camera_motion(self.id(), &payload).await
    }
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct MotionState {
    /// Motion is valid when `motion_report` property is present, invalid when absent.
    #[deprecated]
    pub motion_valid: bool,
    pub motion_report: Option<MotionReport>,
}

#[derive(Debug, Deserialize)]
pub struct MotionReport {
    /// Last time the value of this property is changed.
    pub changed: String,
    /// `true` if motion is detected/
    pub motion: bool,
}

#[derive(Debug, Deserialize)]
pub struct Sensitivity {
    pub status: SetStatus,
    /// Sensitivity of the sensor. Value in the range `0` to `sensitivity_max`.
    pub sensitivity: usize,
    /// Maximum value of the sensitivity configuration attribute.
    pub sensitivity_max: Option<usize>,
}

#[derive(Debug)]
pub struct Temperature<'a> {
    api: &'a BridgeClient,
    data: TemperatureData,
}

impl<'a> Temperature<'a> {
    pub fn new(api: &'a BridgeClient, data: TemperatureData) -> Self {
        Temperature { api, data }
    }

    pub fn data(&self) -> &TemperatureData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub async fn send(
        &self,
        commands: &[BasicCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_temperature(self.id(), &payload).await
    }
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct TemperatureState {
    #[deprecated]
    pub temperature: f32,
    #[deprecated]
    pub temperature_valid: bool,
    pub temperature_report: Option<TemperatureReport>,
}

#[derive(Debug, Deserialize)]
pub struct TemperatureReport {
    /// Last time the value of this property is changed.
    pub changed: String,
    pub temperature: f32,
}

#[derive(Debug)]
pub struct LightLevel<'a> {
    api: &'a BridgeClient,
    data: LightLevelData,
}

impl<'a> LightLevel<'a> {
    pub fn new(api: &'a BridgeClient, data: LightLevelData) -> Self {
        LightLevel { api, data }
    }

    pub fn data(&self) -> &LightLevelData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub async fn send(
        &self,
        commands: &[BasicCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_light_level(self.id(), &payload).await
    }
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct LightLevelState {
    #[deprecated]
    pub light_level: usize,
    #[deprecated]
    pub light_level_valid: bool,
    pub light_level_report: Option<LightLevelReport>,
}

#[derive(Debug, Deserialize)]
pub struct LightLevelReport {
    /// Last time the value of this property is changed.
    pub changed: String,
    /// Light level in `10000*log10(lux) + 1` measured by sensor.
    /// Logarithmic scale used because the human eye adjusts to light levels and small changes at
    /// low lux levels are more noticeable than at high lux levels.
    /// This allows use of linear scale configuration sliders.
    pub light_level: usize,
}

#[derive(Debug)]
pub struct Geolocation<'a> {
    api: &'a BridgeClient,
    data: GeolocationData,
}

impl<'a> Geolocation<'a> {
    pub fn new(api: &'a BridgeClient, data: GeolocationData) -> Self {
        Geolocation { api, data }
    }

    pub fn data(&self) -> &GeolocationData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub async fn send(
        &self,
        commands: &[GeolocationCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_geolocation(self.id(), &payload).await
    }
}

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

#[derive(Debug)]
pub struct GeofenceClient<'a> {
    api: &'a BridgeClient,
    data: GeofenceClientData,
}

impl<'a> GeofenceClient<'a> {
    pub fn new(api: &'a BridgeClient, data: GeofenceClientData) -> Self {
        GeofenceClient { api, data }
    }

    pub fn data(&self) -> &GeofenceClientData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub async fn send(
        &self,
        commands: &[GeofenceClientCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_geofence_client(self.id(), &payload).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GeofenceClientData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    pub name: String,
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
