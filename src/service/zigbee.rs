use super::{
    device::SetStatus,
    resource::{ResourceIdentifier, ResourceType},
};
use crate::{
    api::{BridgeClient, HueAPIError},
    command::{merge_commands, ZigbeeConnectivityCommand, ZigbeeDeviceDiscoveryCommand},
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ZigbeeConnectivity<'a> {
    api: &'a BridgeClient,
    data: ZigbeeConnectivityData,
}

impl<'a> ZigbeeConnectivity<'a> {
    pub fn new(api: &'a BridgeClient, data: ZigbeeConnectivityData) -> Self {
        ZigbeeConnectivity { api, data }
    }

    pub fn data(&self) -> &ZigbeeConnectivityData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub fn status(&self) -> ZigbeeStatus {
        self.data.status
    }

    pub async fn send(
        &self,
        commands: &[ZigbeeConnectivityCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_zigbee_connectivity(self.id(), &payload).await
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

    pub fn id(&self) -> &String {
        &self.data.id
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

#[derive(Debug)]
pub struct ZigbeeDeviceDiscovery<'a> {
    api: &'a BridgeClient,
    data: ZigbeeDeviceDiscoveryData,
}

impl<'a> ZigbeeDeviceDiscovery<'a> {
    pub fn new(api: &'a BridgeClient, data: ZigbeeDeviceDiscoveryData) -> Self {
        ZigbeeDeviceDiscovery { api, data }
    }

    pub fn data(&self) -> &ZigbeeDeviceDiscoveryData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub fn status(&self) -> ZigbeeDeviceDiscoveryStatus {
        self.data.status
    }

    pub async fn send(
        &self,
        commands: &[ZigbeeDeviceDiscoveryCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api
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
    /// If recently changed (`status`: [ZigbeeChannelStatus::Changing]), the value will reflect the channel that is currently being changed to.
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
