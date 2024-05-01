use crate::{
    api::HueAPIError,
    command::{merge_commands, HomeKitCommand, MatterCommand},
    service::{Bridge, ResourceIdentifier, ResourceType},
};
use serde::Deserialize;

/// An Apple HomeKit device.
#[derive(Debug)]
pub struct HomeKit<'a> {
    bridge: &'a Bridge,
    data: HomeKitData,
}

impl<'a> HomeKit<'a> {
    pub fn new(bridge: &'a Bridge, data: HomeKitData) -> Self {
        HomeKit { bridge, data }
    }

    pub fn data(&self) -> &HomeKitData {
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
        commands: &[HomeKitCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_homekit(self.id(), &payload).await
    }
}

/// Internal representation of a [HomeKit].
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

impl HomeKitData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::HomeKit,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HomeKitStatus {
    Paired,
    Pairing,
    Unpaired,
}

/// A virtual device representing interoperating
/// [Matter](https://csa-iot.org/all-solutions/matter/) devices.
#[derive(Debug)]
pub struct Matter<'a> {
    bridge: &'a Bridge,
    data: MatterData,
}

impl<'a> Matter<'a> {
    pub fn new(bridge: &'a Bridge, data: MatterData) -> Self {
        Matter { bridge, data }
    }

    pub fn data(&self) -> &MatterData {
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
        commands: &[MatterCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_matter(self.id(), &payload).await
    }
}

/// Internal representation of the [Matter] interop interface.
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

impl MatterData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Matter,
        }
    }
}

/// A virtual device representing the network of
/// [Matter](https://csa-iot.org/all-solutions/matter/) devices.
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

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }
}

/// Internal representation of a [MatterFabric].
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

impl MatterFabricData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::MatterFabric,
        }
    }
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
