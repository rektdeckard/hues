use crate::{
    api::HueAPIError,
    command::{merge_commands, ZigbeeConnectivityCommand, ZigbeeDeviceDiscoveryCommand},
    service::{Bridge, ResourceIdentifier, ResourceType, SetStatus},
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ZigbeeConnectivity<'a> {
    bridge: &'a Bridge,
    data: ZigbeeConnectivityData,
}

impl<'a> ZigbeeConnectivity<'a> {
    pub fn new(bridge: &'a Bridge, data: ZigbeeConnectivityData) -> Self {
        ZigbeeConnectivity { bridge, data }
    }

    pub fn data(&self) -> &ZigbeeConnectivityData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn status(&self) -> ZigbeeStatus {
        self.data.status
    }

    pub async fn send(
        &self,
        commands: &[ZigbeeConnectivityCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge
            .api
            .put_zigbee_connectivity(self.id(), &payload)
            .await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ZigbeeConnectivityData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Current device communication state with the bridge
    pub status: ZigbeeStatus,
    pub mac_address: String,
    pub channel: Option<ZigbeeChannelState>,
    /// Extended pan id of the zigbee network.
    pub extended_pan_id: Option<String>,
}

impl ZigbeeConnectivityData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::ZigbeeConnectivity,
        }
    }
}

#[derive(Debug)]
pub struct ZGPConnectivity {
    data: ZGPConnectivityData,
}

impl ZGPConnectivity {
    pub fn new(data: ZGPConnectivityData) -> Self {
        ZGPConnectivity { data }
    }

    pub fn data(&self) -> &ZGPConnectivityData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn status(&self) -> ZigbeeStatus {
        self.data.status
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ZGPConnectivityData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Current device communication state with the bridge
    pub status: ZigbeeStatus,
    pub source_id: String,
}

impl ZGPConnectivityData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::ZGPConnectivity,
        }
    }
}

#[derive(Debug)]
pub struct ZigbeeDeviceDiscovery<'a> {
    bridge: &'a Bridge,
    data: ZigbeeDeviceDiscoveryData,
}

impl<'a> ZigbeeDeviceDiscovery<'a> {
    pub fn new(bridge: &'a Bridge, data: ZigbeeDeviceDiscoveryData) -> Self {
        ZigbeeDeviceDiscovery { bridge, data }
    }

    pub fn data(&self) -> &ZigbeeDeviceDiscoveryData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn status(&self) -> ZigbeeDeviceDiscoveryStatus {
        self.data.status
    }

    pub async fn send(
        &self,
        commands: &[ZigbeeDeviceDiscoveryCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge
            .api
            .put_zigbee_device_discovery(self.id(), &payload)
            .await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ZigbeeDeviceDiscoveryData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Current device communication state with the bridge
    pub status: ZigbeeDeviceDiscoveryStatus,
}

impl ZigbeeDeviceDiscoveryData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::ZigbeeDeviceDiscovery,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ZigbeeDeviceDiscoveryStatus {
    Active,
    Ready,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ZigbeeStatus {
    /// The device has been recently been available.
    Connected,
    /// The device has not been recently been available.
    Disconnected,
    /// The device is powered off or has network issues.
    ConnectivityIssue,
    /// The device only talks to bridge.
    UnidirectionalIncoming,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ZigbeeChannelState {
    pub status: SetStatus,
    /// Current value of the zigbee channel.
    /// If recently changed (`status`: [SetStatus::Changing]), the value will reflect the channel that is currently being changed to.
    pub value: Option<ZigbeeChannel>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ZigbeeChannel {
    #[serde(rename = "channel_11")]
    Channel11,
    #[serde(rename = "channel_15")]
    Channel15,
    #[serde(rename = "channel_20")]
    Channel20,
    #[serde(rename = "channel_25")]
    Channel25,
    NotConfigured,
}
