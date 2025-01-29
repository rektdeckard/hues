use crate::{
    api::HueAPIError,
    command::{merge_commands, EntertainmentConfigurationCommand},
    service::{BasicMetadata, BasicStatus, Bridge, ResourceIdentifier, ResourceType},
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct EntertainmentConfiguration<'a> {
    bridge: &'a Bridge,
    data: EntertainmentConfigurationData,
}

impl<'a> EntertainmentConfiguration<'a> {
    pub fn new(bridge: &'a Bridge, data: EntertainmentConfigurationData) -> Self {
        EntertainmentConfiguration { bridge, data }
    }

    pub fn data(&self) -> &EntertainmentConfigurationData {
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
        commands: &[EntertainmentConfigurationCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge
            .api
            .put_entertainment_configuration(self.id(), &payload)
            .await
    }

    #[cfg(feature = "streaming")]
    pub async fn open_stream(&self) {}
}

/// Internal representation of an [EntertainmentConfiguration].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntertainmentConfigurationData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    pub metadata: BasicMetadata,
    /// Friendly name of the entertainment configuration.
    #[deprecated = "use `metadata.name`"]
    pub name: Option<String>,
    /// Defines for which type of application this channel assignment was optimized for.
    pub configuration_type: EntertainmentConfigurationType,
    /// Read-only field reporting if the stream is active or not.
    pub status: BasicStatus,
    /// Expected value is of a ResourceIdentifier of the type
    /// [ResourceType::AuthV1] i.e. an application id, only available if status
    /// is active.
    pub active_streamer: Option<ResourceIdentifier>,
    pub stream_proxy: StreamProxy,
    /// Holds the channels. Each channel groups segments of one or more lights.
    pub channels: Vec<EntertainmentChannel>,
    /// Entertainment services of the lights that are in the zone have locations.
    pub locations: EntertainmentServiceLocations,
    /// List of light services that belong to this entertainment configuration.
    #[deprecated = "resolve via entertainment services in locations object"]
    pub light_services: Option<Vec<ResourceIdentifier>>,
}

impl EntertainmentConfigurationData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::EntertainmentConfiguration,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntertainmentConfigurationType {
    /// Channels are organized around content from a screen.
    Screen,
    /// Channels are organized around content from one or several monitors.
    Monitor,
    /// Channels are organized for music synchronization.
    Music,
    /// Channels are organized to provide 3d spatial effects.
    #[serde(rename = "3dspace")]
    Space3D,
    #[serde(other)]
    /// General use-case.
    Other,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamProxy {
    /// Proxymode used for this group.
    pub mode: StreamProxyMode,
    /// Reference to the device acting as proxy.
    /// The proxy node relays the entertainment traffic and should be located in or close to all entertainment lamps in this group.
    /// The node set by the application ([StreamProxyMode::Manual]) resp selected by the bridge ([StreamProxyMode::Auto]).
    /// Writing sets `mode` to [StreamProxyMode::Manual]. Is not allowed to be combined with [StreamProxyMode::Auto].
    /// Can be type [ResourceType::Bridge] or [ResourceType::ZigbeeConnectivity].
    pub node: ResourceIdentifier,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamProxyMode {
    Auto,
    Manual,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntertainmentChannel {
    /// Bridge assigns a number upon creation. This is the number to be used by the HueStream API when addressing the channel
    pub channel_id: u8,
    /// xyz position of this channel. It is the average position of its members.
    pub position: Position,
    /// List that references segments that are members of that channel.
    pub members: Vec<SegmentReference>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SegmentReference {
    pub service: ResourceIdentifier,
    pub index: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntertainmentServiceLocations {
    pub service_locations: Vec<EntertainmentServiceLocation>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntertainmentServiceLocation {
    pub service: ResourceIdentifier,
    #[deprecated = "use `positions`"]
    /// Describes the location of the service.
    pub position: Option<Position>,
    /// Describes the location of the service.
    pub positions: Vec<Position>,
    /// Relative equalization factor applied to the entertainment service, to compensate for differences in brightness in the entertainment configuration.
    /// Value cannot be `0`, writing `0` changes it to lowest possible value.
    pub equalization_factor: f32,
}

#[derive(Debug)]
pub struct Entertainment {
    data: EntertainmentData,
}

impl Entertainment {
    pub fn new(data: EntertainmentData) -> Self {
        Entertainment { data }
    }

    pub fn data(&self) -> &EntertainmentData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntertainmentData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Indicates if a lamp can be used for entertainment streaming as renderer.
    pub renderer: bool,
    /// Indicates which light service is linked to this entertainment service.
    pub renderer_reference: Option<ResourceIdentifier>,
    /// Indicates if a lamp can be used for entertainment streaming as a proxy node.
    pub proxy: bool,
    /// Indicates if a lamp can handle the equalization factor to dimming maximum brightness in a stream.
    pub equalizer: bool,
    /// Indicates the maximum number of parallel streaming sessions the bridge supports.
    pub max_streams: Option<usize>,
    /// Holds all parameters concerning the segmentations capabilities of a device.
    pub segments: Option<SegmentData>,
}

impl EntertainmentData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Entertainment,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SegmentData {
    /// Defines if the segmentation of the device are configurable or not.
    pub configurable: bool,
    pub max_segments: usize,
    /// Contains the segments configuration of the device for entertainment purposes.
    /// A device can be segmented in a single way.
    pub segments: Vec<Segment>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Segment {
    pub start: usize,
    pub length: usize,
}
