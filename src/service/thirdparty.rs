use super::resource::{ResourceIdentifier, ResourceType};
use crate::{
    api::{BridgeClient, HueAPIError},
    command::{merge_commands, HomeKitCommand},
    MatterCommand,
};
use serde::Deserialize;

#[derive(Debug)]
pub struct HomeKit<'a> {
    api: &'a BridgeClient,
    data: HomeKitData,
}

impl<'a> HomeKit<'a> {
    pub fn new(api: &'a BridgeClient, data: HomeKitData) -> Self {
        HomeKit { api, data }
    }

    pub fn data(&self) -> &HomeKitData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub async fn send(
        &self,
        commands: &[HomeKitCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_homekit(self.id(), &payload).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HomeKitData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Read only field indicating whether homekit is already paired, currently open for pairing, or unpaired.

    /// Transitions:
    /// - [HomeKitStatus::Unpaired] to [HomeKitStatus::Pairing]: pushlink button press or power cycle.
    /// - [HomeKitStatus::Pairing] to [HomeKitStatus::Paired]: through HAP.
    /// - [HomeKitStatus::Pairing] to [HomeKitStatus::Unpaired]: >10 minutes spent attempting to pair.
    /// - [HomeKitStatus::Paired] > [HomeKitStatus::Unpaired]: homekit reset.
    pub status: HomeKitStatus,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HomeKitStatus {
    Paired,
    Pairing,
    Unpaired,
}

#[derive(Debug)]
pub struct Matter<'a> {
    api: &'a BridgeClient,
    data: MatterData,
}

impl<'a> Matter<'a> {
    pub fn new(api: &'a BridgeClient, data: MatterData) -> Self {
        Matter { api, data }
    }

    pub fn data(&self) -> &MatterData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub async fn send(
        &self,
        commands: &[MatterCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_matter(self.id(), &payload).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct MatterData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Maximum number of fabrics that can exist at a time.
    pub max_fabrics: usize,
    /// Indicates whether a physical QR code is present.
    pub has_qr_code: bool,
}

#[derive(Debug)]
pub struct MatterFabric {
    data: MatterFabricData,
}

impl MatterFabric {
    pub fn new(data: MatterFabricData) -> Self {
        MatterFabric { data }
    }

    pub fn data(&self) -> &MatterFabricData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct MatterFabricData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Only a fabric with status [MatterFabricStatus::Paired] has some `fabric_data`.
    pub status: MatterFabricStatus,
    /// Human readable context to identify fabric.
    pub fabric_data: Option<FabricData>,
    /// UTC date and time when the fabric association was created.
    pub creation_time: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MatterFabricStatus {
    Pending,
    #[serde(rename = "timedout")]
    TimedOut,
    Paired,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FabricData {
    pub label: String,
    /// Matter vendor id of entity that created the fabric association.
    pub vendor_id: usize,
}
