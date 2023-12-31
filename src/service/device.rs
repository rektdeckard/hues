use super::resource::{ResourceIdentifier, ResourceType};
use crate::{
    api::HueAPIError,
    command::{merge_commands, DeviceCommand},
    Bridge,
};
use serde::{Deserialize, Serialize};

pub struct Device<'a> {
    bridge: &'a Bridge,
    data: DeviceData,
}

impl<'a> Device<'a> {
    pub fn new(bridge: &'a Bridge, data: DeviceData) -> Self {
        Device { bridge, data }
    }

    pub fn data(&self) -> &DeviceData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn name(&self) -> &str {
        &self.data.metadata.name
    }

    pub fn archetype(&self) -> ProductArchetype {
        self.data.metadata.archetype
    }

    pub async fn identify(&self) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        self.send(&[DeviceCommand::Identify]).await
    }

    pub async fn send(
        &self,
        commands: &[DeviceCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_device(self.id(), &payload).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeviceData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    pub product_data: ProductData,
    /// Additional metadata including a user given name.
    pub metadata: DeviceMetadata,
    pub usertest: Option<UserTest>,
    /// References all services providing control and state of the device.
    pub services: Vec<ResourceIdentifier>,
}

impl DeviceData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Device,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProductData {
    /// Unique identification of device model.
    pub model_id: String,
    /// Name of device manufacturer.
    pub manufacturer_name: String,
    /// Name of the product.
    pub product_name: String,
    /// Archetype of the product
    pub product_archetype: ProductArchetype,
    /// This device is Hue certified
    pub certified: bool,
    /// Software version of the product
    pub software_version: String,
    /// Hardware type; identified by Manufacturer code and ImageType
    pub hardware_platform_type: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductArchetype {
    Bollard,
    BridgeV2,
    CandleBulb,
    CeilingHorizontal,
    CeilingRound,
    CeilingSquare,
    CeilingTube,
    ChristmasTree,
    ClassicBulb,
    DoubleSpot,
    EdisonBulb,
    EllipseBulb,
    FlexibleLamp,
    FloodBulb,
    FloorLantern,
    FloorShade,
    GroundSpot,
    HueBloom,
    HueCentris,
    HueGo,
    HueIris,
    HueLightstrip,
    HueLightstripPc,
    HueLightstripTv,
    HuePlay,
    HueSigne,
    HueTube,
    LargeGlobeBulb,
    LusterBulb,
    PendantLong,
    PendantRound,
    PendantSpot,
    Plug,
    RecessedCeiling,
    RecessedFloor,
    SingleSpot,
    SmallGlobeBulb,
    SpotBulb,
    StringLight,
    SultanBulb,
    TableShade,
    TableWash,
    TriangleBulb,
    UnknownArchetype,
    VintageBulb,
    VintageCandleBulb,
    WallLantern,
    WallShade,
    WallSpot,
    WallWasher,
}
#[derive(Clone, Debug, Deserialize)]
pub struct DeviceMetadata {
    /// Human readable name of a resource.
    pub name: String,
    /// Product archetype.
    pub archetype: ProductArchetype,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserTest {
    pub status: UserTestStatus,
    /// Activates or extends user usertest mode of device for 120 seconds.
    /// `false` deactivates usertest mode. In usertest mode, devices report
    /// changes in state faster and indicate state changes on device LED.
    pub usertest: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserTestStatus {
    Set,
    Changing,
}

#[derive(Debug)]
pub struct DevicePower {
    pub data: DevicePowerData,
}

impl DevicePower {
    pub fn new(data: DevicePowerData) -> Self {
        DevicePower { data }
    }

    pub fn data(&self) -> &DevicePowerData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn battery_state(&self) -> Option<BatteryState> {
        self.data.power_state.battery_state
    }

    pub fn battery_level(&self) -> Option<f32> {
        self.data.power_state.battery_level
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DevicePowerData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    pub power_state: PowerState,
}

impl DevicePowerData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::DevicePower,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PowerState {
    battery_state: Option<BatteryState>,
    battery_level: Option<f32>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BatteryState {
    Normal,
    Low,
    Critical,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetStatus {
    Set,
    Changing,
}

#[derive(Debug, Deserialize)]
pub struct DeviceSoftwareUpdateData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    pub state: SoftwareUpdateStatus,
    pub problems: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SoftwareUpdateStatus {
    NoUpdate,
    UpdatePending,
    Installing,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BasicMetadata {
    pub name: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BasicStatus {
    Active,
    Inactive,
}
