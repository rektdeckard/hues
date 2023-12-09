use std::default;

use serde::{Deserialize, Serialize};

use crate::{
    api::{HueAPI, HueAPIError, V2},
    Bridge,
};

pub struct CommandBuilder<'b> {
    bridge: &'b Bridge<'b>,
    commands: Vec<CommandType>,
}

impl<'b> CommandBuilder<'b> {
    pub fn new(bridge: &'b Bridge) -> Self {
        CommandBuilder {
            bridge,
            commands: vec![],
        }
    }

    pub fn identify(mut self, id: impl Into<String>) -> Self {
        self.commands
            .push(CommandType::Light(id.into(), LightCommand::Identify));
        self
    }

    pub async fn send(&self) -> Result<(), HueAPIError> {
        for cmd in &self.commands {
            match cmd {
                CommandType::Light(id, lc) => match lc {
                    LightCommand::Identify => {
                        self.bridge.api.identify_light(id).await?;
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }

        Ok(())
    }
}

pub enum CommandType {
    BehaviorInstance(BehaviorInstanceCommand),
    Bridge(BridgeCommand),
    Button(ButtonCommand),
    CameraMotion(CameraMotionCommand),
    Contact(ContactCommand),
    Device(DeviceCommand),
    DevicePower(DevicePowerCommand),
    Entertainment(EntertainmentCommand),
    EntertainmentConfiguration(EntertainmentConfigurationCommand),
    GeofenceClient(GeofenceClientCommand),
    Geolocation(GeolocationCommand),
    GroupedLight(GroupedLightCommand),
    HomeKit(HomeKitCommand),
    Light(String, LightCommand),
    LightLevel(String, LightLevelCommand),
    Matter(MatterCommand),
    MatterFabric(MatterFabricCommand),
    Motion(MotionCommand),
    RelativeRotary(RelativeRotaryCommand),
    Room(RoomCommand),
    Scene(SceneCommand),
    SmartScene(SmartSceneCommand),
    Tamper(TamperCommand),
    Temperature(TemperatureCommand),
    ZGPConnectivity(ZGPConnectivityCommand),
    ZigbeeConnectivity(ZigbeeConnectivityCommand),
    ZigbeeDeviceDiscovery(ZigbeeDeviceDiscoveryCommand),
    Zone(ZoneCommand),
}

pub struct BehaviorInstanceCommand;

pub struct BridgeCommand;

pub struct ButtonCommand;

pub struct CameraMotionCommand;

pub struct ContactCommand;

pub struct DeviceCommand;

pub struct DevicePowerCommand;

pub struct EntertainmentCommand;

pub struct EntertainmentConfigurationCommand;

pub struct GeofenceClientCommand;

pub struct GeolocationCommand;

pub struct GroupedLightCommand;

pub struct HomeKitCommand;

pub enum LightCommand {
    Identify,
    Power(bool),
}

#[derive(Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceIdentifyType {
    /// Performs Zigbee LED identification cycles for 5 seconds.
    Bridge,
    /// Perform one breathe cycle.
    #[default]
    Lights,
    /// Perform LED identification cycles for 15 seconds.
    Sensors,
}

pub struct LightLevelCommand;

pub struct MatterCommand;

pub struct MatterFabricCommand;

pub struct MotionCommand;

pub struct RelativeRotaryCommand;

pub struct RoomCommand;

pub struct SceneCommand;

pub struct SmartSceneCommand;

pub struct TamperCommand;

pub struct TemperatureCommand;

pub struct ZGPConnectivityCommand;

pub struct ZigbeeConnectivityCommand;

pub struct ZigbeeDeviceDiscoveryCommand;

pub struct ZoneCommand;
