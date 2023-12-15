use super::{
    control::{Button, RelativeRotary},
    device::{Device, DevicePower},
    group::Group,
    light::Light,
    resource::{ResourceIdentifier, ResourceType},
    scene::{Scene, SceneBuilder},
    sensor::{
        CameraMotion, GeofenceClient, GeofenceClientBuilder, Geolocation, LightLevel, Motion,
        Temperature,
    },
    thirdparty::{HomeKit, Matter, MatterFabric},
    zigbee::{ZGPConnectivity, ZigbeeConnectivity, ZigbeeDeviceDiscovery},
    zone::{Home, Room, Zone, ZoneBuilder},
};
use crate::{
    api::{BridgeClient, HueAPIError, Version},
    command::CommandBuilder,
    ButtonData, DeviceData, DevicePowerData, DeviceSoftwareUpdateData, GeofenceClientData,
    GeolocationData, GroupData, HomeData, HomeKitData, LightData, LightLevelData, MatterData,
    MatterFabricData, MotionData, RelativeRotaryData, Resource, SceneData, TemperatureData,
    ZGPConnectivityData, ZigbeeConnectivityData, ZigbeeDeviceDiscoveryData, ZoneData,
};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use std::{
    net::IpAddr,
    sync::{Mutex, MutexGuard},
    time::Duration,
};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub enum BridgeDiscoveryError {
    NotFound,
    MDNSUnavailable,
}

#[derive(Debug)]
pub enum BridgeBuildError {
    NoIp,
    NoAppKey,
}

#[derive(Debug)]
pub enum BridgeUserError {
    UnableToCreate,
}

#[derive(Debug, Default)]
pub(crate) struct BridgeCache {
    data: Option<BridgeData>,
    buttons: HashMap<String, ButtonData>,
    devices: HashMap<String, DeviceData>,
    geofence_clients: HashMap<String, GeofenceClientData>,
    geolocations: HashMap<String, GeolocationData>,
    groups: HashMap<String, GroupData>,
    homes: HashMap<String, HomeData>,
    homekits: HashMap<String, HomeKitData>,
    lights: HashMap<String, LightData>,
    light_levels: HashMap<String, LightLevelData>,
    matters: HashMap<String, MatterData>,
    matter_fabrics: HashMap<String, MatterFabricData>,
    motions: HashMap<String, MotionData>,
    motion_cameras: HashMap<String, MotionData>,
    power: HashMap<String, DevicePowerData>,
    rooms: HashMap<String, ZoneData>,
    rotaries: HashMap<String, RelativeRotaryData>,
    scenes: HashMap<String, SceneData>,
    swu: HashMap<String, DeviceSoftwareUpdateData>,
    temps: HashMap<String, TemperatureData>,
    zigbee_conns: HashMap<String, ZigbeeConnectivityData>,
    zigbee_dds: HashMap<String, ZigbeeDeviceDiscoveryData>,
    zgp_conns: HashMap<String, ZGPConnectivityData>,
    zones: HashMap<String, ZoneData>,
}

/// Core structure representing a Hue Bridge device interface.
pub struct Bridge {
    pub(crate) api: Box<BridgeClient>,
    cache: Arc<Mutex<BridgeCache>>,
    listener: Option<JoinHandle<()>>,
}

impl Bridge {
    pub fn new(addr: impl Into<IpAddr>, app_key: impl Into<String>) -> Self {
        let api = BridgeClient::new(addr.into(), app_key.into());
        Bridge {
            api: Box::new(api),
            cache: Arc::new(Mutex::new(BridgeCache::default())),
            listener: None,
        }
    }

    fn from_api(api: BridgeClient) -> Self {
        Bridge {
            api: Box::new(api),
            cache: Arc::new(Mutex::new(BridgeCache::default())),
            listener: None,
        }
    }

    pub async fn discover() -> Result<BridgeBuilder, BridgeDiscoveryError> {
        BridgeBuilder::discover().await
    }

    pub async fn listen(mut self, heartbeat: Duration) -> Self {
        let api = self.api.clone();
        let cache = self.cache.clone();

        if let Ok(data) = api.get_resources().await {
            Bridge::insert_to_cache(&mut cache.lock().unwrap(), data)
        }

        self.listener = Some(tokio::spawn(async move {
            let mut first_tick = true;
            let mut interval = tokio::time::interval(heartbeat);
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

            loop {
                if first_tick {
                    first_tick = false;
                } else {
                    if let Ok(data) = api.get_resources().await {
                        Bridge::insert_to_cache(&mut cache.lock().unwrap(), data)
                    }
                }
                interval.tick().await;
            }
        }));

        self
    }

    pub fn unlisten(&mut self) {
        if let Some(handle) = &self.listener {
            handle.abort();
        }
        self.listener = None;
    }

    pub fn command(&self) -> CommandBuilder {
        CommandBuilder::new(&self)
    }

    pub async fn create_app(
        &mut self,
        app_name: impl Into<String>,
        instance_name: impl Into<String>,
    ) -> Result<String, HueAPIError> {
        self.api.create_app(app_name, instance_name).await
    }

    pub fn config(&self) -> Option<BridgeData> {
        self.cache
            .lock()
            .expect("lock cache")
            .data
            .as_ref()
            .map(|d| d.clone())
    }

    fn insert_to_cache(cache: &mut MutexGuard<'_, BridgeCache>, data: Vec<Resource>) {
        for res in data {
            match res {
                // Resource::AuthV1 => {}
                Resource::Bridge(d) => {
                    cache.data = Some(d);
                }
                Resource::BridgeHome(d) => {
                    cache.homes.insert(d.id.clone(), d);
                }
                Resource::Button(d) => {
                    cache.buttons.insert(d.id.clone(), d);
                }
                Resource::CameraMotion(d) => {
                    cache.motion_cameras.insert(d.id.clone(), d);
                }
                // Resource::Contact => {}
                Resource::Device(d) => {
                    cache.devices.insert(d.id.clone(), d);
                }
                Resource::DevicePower(d) => {
                    cache.power.insert(d.id.clone(), d);
                }
                Resource::DeviceSoftwareUpdate(d) => {
                    cache.swu.insert(d.id.clone(), d);
                }
                Resource::GeofenceClient(d) => {
                    cache.geofence_clients.insert(d.id.clone(), d);
                }
                Resource::Geolocation(d) => {
                    cache.geolocations.insert(d.id.clone(), d);
                }
                Resource::Group(d) => {
                    cache.groups.insert(d.id.clone(), d);
                }
                Resource::HomeKit(d) => {
                    cache.homekits.insert(d.id.clone(), d);
                }
                Resource::Light(d) => {
                    cache.lights.insert(d.id.clone(), d);
                }
                Resource::LightLevel(d) => {
                    cache.light_levels.insert(d.id.clone(), d);
                }
                Resource::Matter(d) => {
                    cache.matters.insert(d.id.clone(), d);
                }
                Resource::MatterFabric(d) => {
                    cache.matter_fabrics.insert(d.id.clone(), d);
                }
                Resource::Motion(d) => {
                    cache.motions.insert(d.id.clone(), d);
                }
                Resource::Room(d) => {
                    cache.rooms.insert(d.id.clone(), d);
                }
                Resource::RelativeRotary(d) => {
                    cache.rotaries.insert(d.id.clone(), d);
                }
                Resource::Scene(d) => {
                    cache.scenes.insert(d.id.clone(), d);
                }
                // Resource::SmartScene => {}
                Resource::Temperature(d) => {
                    cache.temps.insert(d.id.clone(), d);
                }
                // Resource::ZigbeeBridgeConnectivity => {}
                Resource::ZigbeeConnectivity(d) => {
                    cache.zigbee_conns.insert(d.id.clone(), d);
                }
                Resource::ZigbeeDeviceDiscovery(d) => {
                    cache.zigbee_dds.insert(d.id.clone(), d);
                }
                Resource::ZGPConnectivity(d) => {
                    cache.zgp_conns.insert(d.id.clone(), d);
                }
                Resource::Zone(d) => {
                    cache.zones.insert(d.id.clone(), d);
                }
                Resource::Unknown(d) => {
                    dbg!("unknown {:?}", &d);
                }
                _ => {
                    dbg!("unimplmented {:?}", &res);
                }
            }
        }
    }

    fn delete_from_cache(cache: &mut MutexGuard<'_, BridgeCache>, data: &Vec<ResourceIdentifier>) {
        let ids_by_type: HashMap<&ResourceType, HashSet<&String>> =
            data.into_iter().fold(Default::default(), |mut acc, r| {
                if !acc.contains_key(&r.rtype) {
                    acc.insert(&r.rtype, Default::default());
                }
                acc.get_mut(&r.rtype).unwrap().insert(&r.rid);
                acc
            });
        for res in ids_by_type.keys() {
            let ids = ids_by_type.get(res).unwrap();
            match res {
                ResourceType::AuthV1 => {
                    todo!()
                }
                ResourceType::BehaviorInstance => {
                    todo!()
                }
                ResourceType::BehaviorScript => {
                    todo!()
                }
                ResourceType::Bridge => {
                    todo!()
                }
                ResourceType::BridgeHome => {
                    cache.data = None;
                }
                ResourceType::Button => {
                    cache.buttons.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::CameraMotion => {
                    cache.motion_cameras.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::Contact => {
                    todo!()
                }
                ResourceType::Device => {
                    cache.devices.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::DevicePower => {
                    cache.power.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::DeviceSoftwareUpdate => {
                    cache.swu.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::Entertainment => {
                    todo!()
                }
                ResourceType::EntertainmentConfiguration => {
                    todo!()
                }
                ResourceType::Geofence => {
                    todo!()
                }
                ResourceType::GeofenceClient => {
                    cache.geofence_clients.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::Geolocation => {
                    cache.geolocations.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::Group => {
                    cache.groups.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::HomeKit => {
                    cache.homekits.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::Light => {
                    cache.lights.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::LightLevel => {
                    cache.light_levels.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::Matter => {
                    cache.matters.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::MatterFabric => {
                    cache.matter_fabrics.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::Motion => {
                    cache.motions.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::PublicImage => {
                    todo!()
                }
                ResourceType::RelativeRotary => {
                    cache.rotaries.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::Room => {
                    cache.rooms.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::Scene => {
                    cache.scenes.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::SmartScene => {
                    todo!()
                }
                ResourceType::Tamper => {
                    todo!()
                }
                ResourceType::Taurus7455 => {
                    todo!()
                }
                ResourceType::Temperature => {
                    cache.temps.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::ZGPConnectivity => {
                    cache.zgp_conns.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::ZigbeeBridgeConnectivity => {
                    todo!()
                }
                ResourceType::ZigbeeConnectivity => {
                    cache.zigbee_conns.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::ZigbeeDeviceDiscovery => {
                    cache.zigbee_dds.retain(|id, _| !ids.contains(&id));
                }
                ResourceType::Zone => {
                    cache.zones.retain(|id, _| !ids.contains(&id));
                }
            }
        }
    }

    /// NOTE: Does not seem to be functioning on the bridge.
    pub async fn refresh(&mut self) -> Result<(), HueAPIError> {
        let data = self.api.get_resources().await?;
        let mut cache = self.cache.lock().expect("could not lock cache");
        Bridge::insert_to_cache(&mut cache, data);
        Ok(())
    }

    pub fn button(&self, id: impl Into<String>) -> Option<Button> {
        self.cache
            .lock()
            .expect("lock cache")
            .buttons
            .get(&id.into())
            .map(|data| Button::new(data.clone()))
    }

    pub fn buttons(&self) -> Vec<Button> {
        self.cache
            .lock()
            .expect("lock cache")
            .buttons
            .iter()
            .map(|(_, data)| Button::new(data.clone()))
            .collect()
    }

    pub fn relative_rotary(&self, id: impl Into<String>) -> Option<RelativeRotary> {
        self.cache
            .lock()
            .expect("lock cache")
            .rotaries
            .get(&id.into())
            .map(|data| RelativeRotary::new(data.clone()))
    }

    pub fn relative_rotaries(&self) -> Vec<RelativeRotary> {
        self.cache
            .lock()
            .expect("lock cache")
            .rotaries
            .iter()
            .map(|(_, data)| RelativeRotary::new(data.clone()))
            .collect()
    }

    pub fn geolocation(&self, id: impl Into<String>) -> Option<Geolocation> {
        self.cache
            .lock()
            .expect("lock cache")
            .geolocations
            .get(&id.into())
            .map(|data| Geolocation::new(&self.api, data.clone()))
    }

    pub fn geolocations(&self) -> Vec<Geolocation> {
        self.cache
            .lock()
            .expect("lock cache")
            .geolocations
            .iter()
            .map(|(_, data)| Geolocation::new(&self.api, data.clone()))
            .collect()
    }

    pub fn geofence_client(&self, id: impl Into<String>) -> Option<GeofenceClient> {
        self.cache
            .lock()
            .expect("lock cache")
            .geofence_clients
            .get(&id.into())
            .map(|data| GeofenceClient::new(&self.api, data.clone()))
    }

    pub fn geofence_clients(&self) -> Vec<GeofenceClient> {
        self.cache
            .lock()
            .expect("lock cache")
            .geofence_clients
            .iter()
            .map(|(_, data)| GeofenceClient::new(&self.api, data.clone()))
            .collect()
    }

    pub async fn create_geofence_client(
        &self,
        builder: GeofenceClientBuilder,
    ) -> Result<GeofenceClient, HueAPIError> {
        let rid = self
            .api
            .post_geofence_client(serde_json::to_value(builder).unwrap())
            .await?;
        let data = self.api.get_geofence_client(rid.rid).await?;
        self.cache
            .lock()
            .expect("lock cache")
            .geofence_clients
            .insert(data.id.clone(), data.clone());
        Ok(GeofenceClient::new(&self.api, data))
    }

    pub async fn delete_geofence_client(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let res = self.api.delete_geofence_client(id).await?;
        Bridge::delete_from_cache(&mut self.cache.lock().expect("lock cache"), &res);
        Ok(res)
    }

    pub async fn homekit(&self, id: impl Into<String>) -> Option<HomeKit> {
        self.cache
            .lock()
            .expect("lock cache")
            .homekits
            .get(&id.into())
            .map(|data| HomeKit::new(&self.api, data.clone()))
    }

    pub async fn homekits(&self) -> Vec<HomeKit> {
        self.cache
            .lock()
            .expect("lock cache")
            .homekits
            .iter()
            .map(|(_, data)| HomeKit::new(&self.api, data.clone()))
            .collect()
    }

    pub async fn matter(&self, id: impl Into<String>) -> Option<Matter> {
        self.cache
            .lock()
            .expect("lock cache")
            .matters
            .get(&id.into())
            .map(|data| Matter::new(&self.api, data.clone()))
    }

    pub async fn matters(&self) -> Vec<Matter> {
        self.cache
            .lock()
            .expect("lock cache")
            .matters
            .iter()
            .map(|(_, data)| Matter::new(&self.api, data.clone()))
            .collect()
    }

    pub async fn matter_fabric(&self, id: impl Into<String>) -> Option<MatterFabric> {
        self.cache
            .lock()
            .expect("lock cache")
            .matter_fabrics
            .get(&id.into())
            .map(|data| MatterFabric::new(data.clone()))
    }

    pub async fn matter_fabrics(&self) -> Vec<MatterFabric> {
        self.cache
            .lock()
            .expect("lock cache")
            .matter_fabrics
            .iter()
            .map(|(_, data)| MatterFabric::new(data.clone()))
            .collect()
    }

    pub async fn delete_matter_fabric(
        &mut self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let res = self.api.delete_matter_fabric(id).await?;
        Bridge::delete_from_cache(&mut self.cache.lock().expect("lock cache"), &res);
        Ok(res)
    }

    // pub async fn device(&self, id: impl Into<String>) -> Result<&'a Device, HueAPIError> {
    //     let data = self.api.get_device(id).await?;
    //     let id = data.id.clone();
    //     let device = Device::new(&self.api, data);

    //     self.devices.insert(id.clone(), device);
    //     self.devices
    //         .get(&id)
    //         .ok_or_else(|| HueAPIError::BadResponse)
    // }

    // pub async fn devices(&self) -> Result<&'a HashMap<String, Device>, HueAPIError> {
    //     let data = self.api.get_devices().await?;
    //     self.devices = data
    //         .into_iter()
    //         .map(|dev| (dev.id.clone(), Device::new(&self.api, dev)))
    //         .collect();
    //     Ok(&self.devices)
    // }

    // pub async fn device_power(
    //     &self,
    //     id: impl Into<String>,
    // ) -> Result<&'a DevicePower, HueAPIError> {
    //     let data = self.api.get_device_power(id).await?;
    //     let id = data.id.clone();
    //     let power = DevicePower::new(data);

    //     self.power.insert(id.clone(), power);
    //     self.power.get(&id).ok_or_else(|| HueAPIError::BadResponse)
    // }

    // pub async fn device_powers(
    //     &self,
    // ) -> Result<&'a HashMap<String, DevicePower>, HueAPIError> {
    //     let data = self.api.get_device_powers().await?;
    //     self.power = data
    //         .into_iter()
    //         .map(|power| (power.id.clone(), DevicePower::new(power)))
    //         .collect();
    //     Ok(&self.power)
    // }

    // pub async fn delete_device(
    //     &mut self,
    //     id: impl Into<String>,
    // ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
    //     let res = self.api.delete_device(id).await?;
    //     let ids = res.iter().map(|rid| &rid.rid).collect::<HashSet<_>>();
    //     self.devices.retain(|id, _| !ids.contains(id));
    //     Ok(res)
    // }

    pub fn group(&self, id: impl Into<String>) -> Option<Group> {
        self.cache
            .lock()
            .expect("lock cache")
            .groups
            .get(&id.into())
            .map(|data| Group::new(&self.api, data.clone()))
    }

    pub fn groups(&self) -> Vec<Group> {
        self.cache
            .lock()
            .expect("lock cache")
            .groups
            .iter()
            .map(|(_, data)| Group::new(&self.api, data.clone()))
            .collect()
    }

    // pub async fn home(&self, id: impl Into<String>) -> Result<&'a Home, HueAPIError> {
    //     let data = self.api.get_bridge_home(id).await?;
    //     let id = data.id.clone();
    //     let home = Home::new(data);

    //     self.homes.insert(id.clone(), home);
    //     self.homes.get(&id).ok_or_else(|| HueAPIError::BadResponse)
    // }

    // pub async fn homes(&self) -> Result<&'a HashMap<String, Home>, HueAPIError> {
    //     let data = self.api.get_bridge_homes().await?;
    //     self.homes = data
    //         .into_iter()
    //         .map(|home| (home.id.clone(), Home::new(home)))
    //         .collect();
    //     Ok(&self.homes)
    // }

    pub fn light(&self, id: impl Into<String>) -> Option<Light> {
        self.cache
            .lock()
            .expect("lock cache")
            .lights
            .get(&id.into())
            .map(|data| Light::new(&self.api, data.clone()))
    }

    pub fn lights(&self) -> Vec<Light> {
        self.cache
            .lock()
            .expect("lock cache")
            .lights
            .iter()
            .map(|(_, data)| Light::new(&self.api, data.clone()))
            .collect()
    }

    // pub async fn motion(&self, id: impl Into<String>) -> Result<Motion, HueAPIError> {
    //     self.api
    //         .get_motion(id)
    //         .await
    //         .and_then(|md| Ok(Motion::new(&self.api, md)))
    // }

    // pub async fn motions(&self) -> Result<Vec<Motion>, HueAPIError> {
    //     let data = self.api.get_motions().await?;
    //     Ok(data
    //         .into_iter()
    //         .map(|md| Motion::new(&self.api, md))
    //         .collect())
    // }

    // pub async fn camera_motion(&self, id: impl Into<String>) -> Result<CameraMotion, HueAPIError> {
    //     self.api
    //         .get_motion(id)
    //         .await
    //         .and_then(|md| Ok(CameraMotion::new(&self.api, md)))
    // }

    // pub async fn camera_motions(&self) -> Result<Vec<CameraMotion>, HueAPIError> {
    //     let data = self.api.get_camera_motions().await?;
    //     Ok(data
    //         .into_iter()
    //         .map(|md| CameraMotion::new(&self.api, md))
    //         .collect())
    // }

    // pub async fn room(&self, id: impl Into<String>) -> Result<&'a Room, HueAPIError> {
    //     let data = self.api.get_room(id).await?;
    //     let id = data.id.clone();
    //     let room = Room::new(&self.api, data);

    //     self.rooms.insert(id.clone(), room);
    //     self.rooms.get(&id).ok_or_else(|| HueAPIError::BadResponse)
    // }

    // pub async fn rooms(&self) -> Result<&'a HashMap<String, Room>, HueAPIError> {
    //     let res = self.api.get_rooms().await?;
    //     self.rooms.extend(
    //         res.into_iter()
    //             .map(|room| (room.id.clone(), Room::new(&self.api, room))),
    //     );
    //     Ok(&self.rooms)
    // }

    // pub async fn create_room(&self, builder: ZoneBuilder) -> Result<&'a Room, HueAPIError> {
    //     let rid = self
    //         .api
    //         .post_room(serde_json::to_value(builder).unwrap())
    //         .await?;
    //     self.room(rid.rid).await
    // }

    // pub async fn delete_room(
    //     &self,
    //     id: impl Into<String>,
    // ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
    //     let res = self.api.delete_room(id).await?;
    //     let ids = res.iter().map(|rid| &rid.rid).collect::<HashSet<_>>();
    //     self.rooms.retain(|id, _| !ids.contains(id));
    //     Ok(res)
    // }

    pub fn scene(&self, id: impl Into<String>) -> Option<Scene> {
        self.cache
            .lock()
            .expect("lock cache")
            .scenes
            .get(&id.into())
            .map(|data| Scene::new(&self.api, data.clone()))
    }

    pub fn scenes(&self) -> Vec<Scene> {
        self.cache
            .lock()
            .expect("lock cache")
            .scenes
            .iter()
            .map(|(_, data)| Scene::new(&self.api, data.clone()))
            .collect()
    }

    pub async fn create_scene(&self, builder: SceneBuilder) -> Result<Scene, HueAPIError> {
        let rid = self
            .api
            .post_scene(serde_json::to_value(builder).unwrap())
            .await?;
        let data = self.api.get_scene(rid.rid).await?;
        self.cache
            .lock()
            .expect("lock cache")
            .scenes
            .insert(data.id.clone(), data.clone());
        Ok(Scene::new(&self.api, data))
    }

    // // pub async fn update_scene(&mut self, )

    pub async fn delete_scene(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let res = self.api.delete_scene(id).await?;
        Bridge::delete_from_cache(&mut self.cache.lock().expect("lock cache"), &res);
        Ok(res)
    }

    // pub async fn light_level(
    //     &self,
    //     id: impl Into<String>,
    // ) -> Result<&'a LightLevel, HueAPIError> {
    //     let data = self.api.get_light_level(id).await?;
    //     let id = data.id.clone();
    //     let ll = LightLevel::new(&self.api, data);

    //     self.light_levels.insert(id.clone(), ll);
    //     self.light_levels
    //         .get(&id)
    //         .ok_or_else(|| HueAPIError::BadResponse)
    // }

    // pub async fn light_levels(
    //     &self,
    // ) -> Result<&'a HashMap<String, LightLevel>, HueAPIError> {
    //     let data = self.api.get_light_levels().await?;
    //     self.light_levels = data
    //         .into_iter()
    //         .map(|data| (data.id.clone(), LightLevel::new(&self.api, data)))
    //         .collect();
    //     Ok(&self.light_levels)
    // }

    // pub async fn temperature(
    //     &self,
    //     id: impl Into<String>,
    // ) -> Result<&'a Temperature, HueAPIError> {
    //     let data = self.api.get_temperature(id).await?;
    //     let id = data.id.clone();
    //     let temperature = Temperature::new(&self.api, data);

    //     self.temps.insert(id.clone(), temperature);
    //     self.temps.get(&id).ok_or_else(|| HueAPIError::BadResponse)
    // }

    // pub async fn temperatures(
    //     &self,
    // ) -> Result<&'a HashMap<String, Temperature>, HueAPIError> {
    //     let temps = self.api.get_temperatures().await?;
    //     self.temps = temps
    //         .into_iter()
    //         .map(|data| (data.id.clone(), Temperature::new(&self.api, data)))
    //         .collect();
    //     Ok(&self.temps)
    // }

    // pub async fn zgp_connectivity(
    //     &self,
    //     id: impl Into<String>,
    // ) -> Result<ZGPConnectivity, HueAPIError> {
    //     let data = self.api.get_zgp_connectivity(id).await?;
    //     Ok(ZGPConnectivity::new(data))
    // }

    // pub async fn zgp_connectivities(&self) -> Result<Vec<ZGPConnectivity>, HueAPIError> {
    //     let data = self.api.get_zgp_connectivities().await?;
    //     Ok(data
    //         .into_iter()
    //         .map(|zigb| ZGPConnectivity::new(zigb))
    //         .collect())
    // }

    // pub async fn zigbee_connectivity(
    //     &self,
    //     id: impl Into<String>,
    // ) -> Result<ZigbeeConnectivity, HueAPIError> {
    //     let data = self.api.get_zigbee_connectivity(id).await?;
    //     Ok(ZigbeeConnectivity::new(&self.api, data))
    // }

    // pub async fn zigbee_connectivities(&self) -> Result<Vec<ZigbeeConnectivity>, HueAPIError> {
    //     let data = self.api.get_zigbee_connectivities().await?;
    //     Ok(data
    //         .into_iter()
    //         .map(|zigb| ZigbeeConnectivity::new(&self.api, zigb))
    //         .collect())
    // }

    // pub async fn zigbee_device_discovery(
    //     &self,
    //     id: impl Into<String>,
    // ) -> Result<ZigbeeDeviceDiscovery, HueAPIError> {
    //     let data = self.api.get_zigbee_device_discovery(id).await?;
    //     Ok(ZigbeeDeviceDiscovery::new(&self.api, data))
    // }

    // pub async fn zigbee_device_discoveries(
    //     &self,
    // ) -> Result<Vec<ZigbeeDeviceDiscovery>, HueAPIError> {
    //     let data = self.api.get_zigbee_device_discoveries().await?;
    //     Ok(data
    //         .into_iter()
    //         .map(|zigb| ZigbeeDeviceDiscovery::new(&self.api, zigb))
    //         .collect())
    // }

    // pub async fn zone(&self, id: impl Into<String>) -> Result<&'a Zone, HueAPIError> {
    //     let data = self.api.get_zone(id).await?;
    //     let id = data.id.clone();
    //     let zone = Zone::new(&self.api, data);

    //     self.zones.insert(id.clone(), zone);
    //     self.zones.get(&id).ok_or_else(|| HueAPIError::BadResponse)
    // }

    // pub async fn zones(&self) -> Result<&'a HashMap<String, Zone>, HueAPIError> {
    //     let res = self.api.get_zones().await?;
    //     self.zones.extend(
    //         res.into_iter()
    //             .map(|zone| (zone.id.clone(), Zone::new(&self.api, zone))),
    //     );
    //     Ok(&self.zones)
    // }

    // pub async fn create_zone(&self, builder: ZoneBuilder) -> Result<&'a Zone, HueAPIError> {
    //     let rid = self
    //         .api
    //         .post_zone(serde_json::to_value(builder).unwrap())
    //         .await?;
    //     self.zone(rid.rid).await
    // }

    // pub async fn delete_zone(
    //     &self,
    //     id: impl Into<String>,
    // ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
    //     let res = self.api.delete_zone(id).await?;
    //     let ids = res.iter().map(|rid| &rid.rid).collect::<HashSet<_>>();
    //     self.zones.retain(|id, _| !ids.contains(id));
    //     Ok(res)
    // }
}

#[derive(Clone, Debug, Deserialize)]
pub struct BridgeData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Unique identifier of the bridge as printed on the device. Lower case (shouldn't it be upper case?)
    pub bridge_id: String,
    pub time_zone: TimeZone,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TimeZone {
    pub time_zone: String,
}

/// Builder structure representing a [Bridge] that is not yet fully configured.
pub struct BridgeBuilder {
    addr: Option<IpAddr>,
    app_key: Option<String>,
    version: Version,
}

impl Default for BridgeBuilder {
    fn default() -> Self {
        BridgeBuilder {
            addr: None,
            app_key: None,
            version: Default::default(),
        }
    }
}

impl BridgeBuilder {
    pub fn new() -> Self {
        BridgeBuilder::default()
    }

    async fn discover_http() -> Result<Self, BridgeDiscoveryError> {
        todo!()
    }

    #[cfg(feature = "mdns")]
    async fn discover_mdns() -> Result<Self, BridgeDiscoveryError> {
        use futures_util::{pin_mut, stream::StreamExt};
        const SERVICE_NAME: &'static str = "_hue._tcp.local";

        let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))
            .unwrap()
            .listen();
        pin_mut!(stream);

        if let Some(Ok(response)) = stream.next().await {
            for rec in response.answers {
                match rec.kind {
                    mdns::RecordKind::A(addr) => {
                        return Ok(BridgeBuilder {
                            addr: Some(addr.into()),
                            ..Default::default()
                        })
                    }
                    _ => {}
                }
            }
            Err(BridgeDiscoveryError::NotFound)
        } else {
            Err(BridgeDiscoveryError::MDNSUnavailable)
        }
    }

    pub async fn discover() -> Result<Self, BridgeDiscoveryError> {
        #[cfg(feature = "mdns")]
        if let Ok(bridge) = BridgeBuilder::discover_mdns().await {
            return Ok(bridge);
        }
        BridgeBuilder::discover_http().await
    }

    pub fn app_key(mut self, key: &str) -> Self {
        self.app_key = Some(key.into());
        self
    }

    pub fn version(mut self, v: Version) -> Self {
        self.version = v;
        self
    }

    pub fn build(self) -> Bridge {
        let addr = self.addr.unwrap_or([0u8, 0, 0, 0].into());
        let app_key = self.app_key.unwrap_or_default();
        let api = if self.version == Version::V2 {
            BridgeClient::new(addr, app_key)
        } else {
            todo!()
        };

        Bridge::from_api(api)
    }
}
