use super::{
    bridge::Bridge,
    resource::{ResourceIdentifier, ResourceType},
};
use crate::{
    api::HueAPIError,
    command::{merge_commands, ZoneCommand},
    Device, Light, Scene,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Zone<'a> {
    bridge: &'a Bridge,
    pub data: ZoneData,
}

impl<'a> Zone<'a> {
    pub fn new(bridge: &'a Bridge, data: ZoneData) -> Self {
        Zone { bridge, data }
    }

    pub fn data(&self) -> &ZoneData {
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

    pub fn archetype(&self) -> ZoneArchetype {
        self.data.metadata.archetype
    }

    pub fn devices(&self) -> Vec<Device> {
        let rids = &self.data.children;
        self.bridge
            .devices()
            .into_iter()
            .filter_map(|d| {
                if rids.contains(&d.rid()) {
                    Some(d)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn lights(&self) -> Vec<Light> {
        self.bridge
            .lights()
            .into_iter()
            .filter(|l| self.data.children.contains(&l.data().owner))
            .collect()
    }

    pub fn scenes(&self) -> Vec<Scene> {
        self.bridge
            .scenes()
            .into_iter()
            .filter(|s| self.rid() == s.group())
            .collect()
    }

    pub fn builder(name: impl Into<String>, archetype: ZoneArchetype) -> ZoneBuilder {
        ZoneBuilder::new(name, archetype)
    }

    pub async fn send(
        &self,
        commands: &[ZoneCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_zone(self.id(), &payload).await
    }
}

#[derive(Debug)]
pub struct Room<'a> {
    bridge: &'a Bridge,
    pub data: ZoneData,
}

impl<'a> Room<'a> {
    pub fn new(bridge: &'a Bridge, data: ZoneData) -> Self {
        Room { bridge, data }
    }

    pub fn data(&self) -> &ZoneData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id().to_owned(),
            rtype: ResourceType::Room,
        }
    }

    pub fn name(&self) -> &str {
        &self.data.metadata.name
    }

    pub fn archetype(&self) -> ZoneArchetype {
        self.data.metadata.archetype
    }

    pub fn devices(&self) -> Vec<Device> {
        let rids = &self.data.children;
        self.bridge
            .devices()
            .into_iter()
            .filter_map(|d| {
                if rids.contains(&d.rid()) {
                    Some(d)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn lights(&self) -> Vec<Light> {
        self.bridge
            .lights()
            .into_iter()
            .filter(|l| self.data.children.contains(&l.data().owner))
            .collect()
    }

    pub fn scenes(&self) -> Vec<Scene> {
        self.bridge
            .scenes()
            .into_iter()
            .filter(|s| self.rid() == s.group())
            .collect()
    }

    pub fn builder(name: impl Into<String>, archetype: ZoneArchetype) -> ZoneBuilder {
        ZoneBuilder::new(name, archetype)
    }

    pub async fn send(
        &self,
        commands: &[ZoneCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_room(self.id(), &payload).await
    }
}

#[derive(Serialize)]
pub struct ZoneBuilder {
    pub metadata: ZoneMetadata,
    pub children: Vec<ResourceIdentifier>,
}

impl ZoneBuilder {
    pub fn new(name: impl Into<String>, archetype: ZoneArchetype) -> Self {
        ZoneBuilder {
            metadata: ZoneMetadata {
                name: name.into(),
                archetype,
            },
            children: vec![],
        }
    }

    pub fn children(mut self, children: Vec<ResourceIdentifier>) -> Self {
        self.children = children;
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZoneData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Child devices/services to group by the derived group.
    pub children: Vec<ResourceIdentifier>,
    /// References all services aggregating control and state of children in the group.
    ///
    /// This includes all services grouped in the group hierarchy given by child relation.
    /// This includes all services of a device grouped in the group hierarchy given by child relation.
    /// Aggregation is per service type, i.e. every service type which can be grouped has a
    /// corresponding definition of grouped type.
    /// Supported `rtype`: [ResourceType::Group]
    pub services: Vec<ResourceIdentifier>,
    /// Configuration for a zone object.
    pub metadata: ZoneMetadata,
}

impl ZoneData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Zone,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZoneMetadata {
    /// Human readable name of a resource.
    pub name: String,
    /// Possible archetypes of a zone.
    pub archetype: ZoneArchetype,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoneArchetype {
    Attic,
    Balcony,
    Barbecue,
    Bathroom,
    Bedroom,
    Carport,
    Closet,
    Computer,
    Dining,
    Downstairs,
    Driveway,
    FrontDoor,
    Garage,
    Garden,
    GuestZone,
    Gym,
    Hallway,
    Home,
    KidsBedroom,
    Kitchen,
    LaundryZone,
    LivingZone,
    Lounge,
    ManCave,
    Music,
    Nursery,
    Office,
    Other,
    Pool,
    Porch,
    Reading,
    Recreation,
    Staircase,
    Storage,
    Studio,
    Terrace,
    Toilet,
    TopFloor,
    Tv,
    Upstairs,
}

#[derive(Debug)]
pub struct Home {
    data: HomeData,
}

impl Home {
    pub fn new(data: HomeData) -> Self {
        Home { data }
    }

    pub fn data(&self) -> &HomeData {
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
pub struct HomeData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Child devices/services to group by the derived group.
    pub children: Vec<ResourceIdentifier>,
    /// References all services aggregating control and state of children in the group.
    ///
    /// This includes all services grouped in the group hierarchy given by child relation.
    /// This includes all services of a device grouped in the group hierarchy given by child relation.
    /// Aggregation is per service type, i.e. every service type which can be grouped has a
    /// corresponding definition of grouped type.
    /// Supported `rtype`: [ResourceType::Group]
    pub services: Vec<ResourceIdentifier>,
}

impl HomeData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::BridgeHome,
        }
    }
}
