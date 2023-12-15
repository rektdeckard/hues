use serde::{Deserialize, Serialize};

use crate::{
    BridgeData, ButtonData, DeviceData, DevicePowerData, DeviceSoftwareUpdateData,
    GeofenceClientData, GeolocationData, GroupData, HomeData, HomeKitData, LightData,
    LightLevelData, MatterData, MatterFabricData, MotionData, RelativeRotaryData, SceneData,
    TemperatureData, ZGPConnectivityData, ZigbeeConnectivityData, ZigbeeDeviceDiscoveryData,
    ZoneData,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Resource {
    #[serde(rename = "auth_v1")]
    AuthV1,
    BehaviorInstance,
    BehaviorScript,
    Bridge(BridgeData),
    BridgeHome(HomeData),
    Button(ButtonData),
    CameraMotion(MotionData),
    Contact,
    Device(DeviceData),
    DevicePower(DevicePowerData),
    DeviceSoftwareUpdate(DeviceSoftwareUpdateData),
    Entertainment,
    EntertainmentConfiguration,
    Geofence,
    GeofenceClient(GeofenceClientData),
    Geolocation(GeolocationData),
    #[serde(rename = "grouped_light")]
    Group(GroupData),
    #[serde(rename = "homekit")]
    HomeKit(HomeKitData),
    Light(LightData),
    LightLevel(LightLevelData),
    Matter(MatterData),
    MatterFabric(MatterFabricData),
    Motion(MotionData),
    PublicImage,
    RelativeRotary(RelativeRotaryData),
    Room(ZoneData),
    Scene(SceneData),
    SmartScene,
    Tamper,
    #[serde(rename = "taurus_7455")]
    Taurus7455,
    Temperature(TemperatureData),
    ZGPConnectivity(ZGPConnectivityData),
    ZigbeeBridgeConnectivity,
    ZigbeeConnectivity(ZigbeeConnectivityData),
    ZigbeeDeviceDiscovery(ZigbeeDeviceDiscoveryData),
    Zone(ZoneData),
    Unknown(UnknownData),
}

#[derive(Debug, Deserialize)]
pub struct UnknownData {
    #[serde(rename = "type")]
    pub rtype: String,
    /// Unique identifier representing a specific resource instance.
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResourceIdentifier {
    /// The unique id of the referenced resource.
    pub rid: String,
    /// The type of the referenced resource.
    pub rtype: ResourceType,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    #[serde(rename = "auth_v1")]
    AuthV1,
    BehaviorInstance,
    BehaviorScript,
    Bridge,
    BridgeHome,
    Button,
    CameraMotion,
    Contact,
    Device,
    DevicePower,
    DeviceSoftwareUpdate,
    Entertainment,
    EntertainmentConfiguration,
    Geofence,
    GeofenceClient,
    Geolocation,
    #[serde(rename = "grouped_light")]
    Group,
    #[serde(rename = "homekit")]
    HomeKit,
    Light,
    LightLevel,
    Matter,
    MatterFabric,
    Motion,
    PublicImage,
    RelativeRotary,
    Room,
    Scene,
    SmartScene,
    Tamper,
    #[serde(rename = "taurus_7455")]
    Taurus7455,
    Temperature,
    ZGPConnectivity,
    ZigbeeBridgeConnectivity,
    ZigbeeConnectivity,
    ZigbeeDeviceDiscovery,
    Zone,
}
