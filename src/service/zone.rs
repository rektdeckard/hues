use super::resource::{ResourceIdentifier, ResourceType};
use crate::{
    api::{BridgeClient, HueAPIError},
    command::{merge_commands, ZoneCommand},
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Zone<'a> {
    api: &'a BridgeClient,
    pub data: ZoneData,
}

impl<'a> Zone<'a> {
    pub fn new(api: &'a BridgeClient, data: ZoneData) -> Self {
        Zone { api, data }
    }

    pub fn data(&self) -> &ZoneData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub fn name(&self) -> &String {
        &self.data.metadata.name
    }

    pub fn archetype(&self) -> ZoneArchetype {
        self.data.metadata.archetype
    }

    pub fn builder(name: impl Into<String>, archetype: ZoneArchetype) -> ZoneBuilder {
        ZoneBuilder::new(name, archetype)
    }

    pub async fn send(
        &self,
        commands: &[ZoneCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_zone(self.id(), &payload).await
    }
}

#[derive(Debug)]
pub struct Room<'a> {
    api: &'a BridgeClient,
    pub data: ZoneData,
}

impl<'a> Room<'a> {
    pub fn new(api: &'a BridgeClient, data: ZoneData) -> Self {
        Room { api, data }
    }

    pub fn data(&self) -> &ZoneData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub fn name(&self) -> &String {
        &self.data.metadata.name
    }

    pub fn archetype(&self) -> ZoneArchetype {
        self.data.metadata.archetype
    }

    pub fn builder(name: impl Into<String>, archetype: ZoneArchetype) -> ZoneBuilder {
        ZoneBuilder::new(name, archetype)
    }

    pub async fn send(
        &self,
        commands: &[ZoneCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_room(self.id(), &payload).await
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneMetadata {
    /// Human readable name of a resource.
    pub name: String,
    /// Possible archetypes of a zone.
    pub archetype: ZoneArchetype,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
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

    pub fn id(&self) -> &String {
        &self.data.id
    }
}

#[derive(Debug, Deserialize, Serialize)]
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
