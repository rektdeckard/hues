use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HueEvent {
    pub id: String,
    #[serde(rename = "creationtime")]
    pub creation_time: String,
    pub data: Vec<HueEventData>,
    #[serde(rename = "type")]
    pub etype: HueEventType,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HueEventType {
    Add,
    Delete,
    Update,
    Error,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum HueEventData {
    #[serde(rename = "auth_v1")]
    AuthV1,
    BehaviorInstance,
    BehaviorScript(serde_json::Value),
    Bridge(serde_json::Value),
    BridgeHome(serde_json::Value),
    Button(serde_json::Value),
    CameraMotion(serde_json::Value),
    Contact(serde_json::Value),
    Device(serde_json::Value),
    DevicePower(serde_json::Value),
    DeviceSoftwareUpdate(serde_json::Value),
    Entertainment,
    EntertainmentConfiguration,
    Geofence,
    GeofenceClient(serde_json::Value),
    Geolocation(serde_json::Value),
    #[serde(rename = "grouped_light")]
    Group(serde_json::Value),
    #[serde(rename = "homekit")]
    HomeKit(serde_json::Value),
    Light(serde_json::Value),
    LightLevel(serde_json::Value),
    Matter(serde_json::Value),
    MatterFabric(serde_json::Value),
    Motion(serde_json::Value),
    PublicImage,
    RelativeRotary(serde_json::Value),
    Room(serde_json::Value),
    Scene(serde_json::Value),
    SmartScene(serde_json::Value),
    Tamper(serde_json::Value),
    #[serde(rename = "taurus_7455")]
    Taurus7455,
    Temperature(serde_json::Value),
    ZGPConnectivity(serde_json::Value),
    ZigbeeBridgeConnectivity,
    ZigbeeConnectivity(serde_json::Value),
    ZigbeeDeviceDiscovery(serde_json::Value),
    Zone(serde_json::Value),
    #[serde(other)]
    Unknown,
}
