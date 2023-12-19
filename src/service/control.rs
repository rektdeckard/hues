use super::resource::{ResourceIdentifier, ResourceType};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Button {
    data: ButtonData,
}

impl Button {
    pub fn new(data: ButtonData) -> Self {
        Button { data }
    }

    pub fn data(&self) -> &ButtonData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn control_id(&self) -> u8 {
        self.data.metadata.control_id
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ButtonData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    /// Metadata describing this resource.
    pub metadata: ButtonMetadata,
    pub button: ButtonState,
}

impl ButtonData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Button,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ButtonMetadata {
    /// Control identifier of the switch which is unique per device.
    /// In combination with type:
    /// - dots Number of dots
    /// – number Number printed on device
    /// – other a logical order of controls in switch
    pub control_id: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ButtonState {
    #[deprecated]
    pub last_event: Option<ButtonEvent>,
    pub button_report: Option<ButtonReport>,
    /// Duration of a light transition or timed effects in ms.
    pub repeat_interval: Option<usize>,
    /// List of all button events that this device supports.
    pub event_values: HashSet<ButtonEvent>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonEvent {
    InitialPress,
    Repeat,
    ShortRelease,
    LongRelease,
    DoubleShortRelease,
    LongPress,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ButtonReport {
    /// Last time the value of this property is updated.
    pub updated: String,
    /// Events which can be sent by a button control.
    pub event: ButtonEvent,
}

#[derive(Debug)]
pub struct RelativeRotary {
    data: RelativeRotaryData,
}

impl RelativeRotary {
    pub fn new(data: RelativeRotaryData) -> Self {
        RelativeRotary { data }
    }

    pub fn data(&self) -> &RelativeRotaryData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RelativeRotaryData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Owner of the service, in case the owner service is deleted, the service also gets deleted.
    pub owner: ResourceIdentifier,
    pub relative_rotary: RelativeRotaryState,
}

impl RelativeRotaryData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::RelativeRotary,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RelativeRotaryState {
    #[deprecated]
    /// Renamed to RelativeRotaryReport. Indicates which type of rotary event is received.
    pub last_event: Option<RelativeRotaryLastEvent>,
    pub rotary_report: Option<RotationReport>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RelativeRotaryLastEvent {
    /// Indicates which type of rotary event is received.
    pub action: RelativeRotaryAction,
    pub rotation: RelativeRotaryRotationState,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelativeRotaryAction {
    Start,
    Repeat,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RelativeRotaryRotationState {
    /// A rotation opposite to the previous rotation will always start with new start command.
    pub direction: RelativeRotaryDirection,
    /// Amount of rotation since previous event in case of repeat,
    /// amount of rotation since start in case of a start_event.
    /// Resolution = `1000` steps / `360` degree rotation.
    pub steps: u16,
    /// Duration of rotation since previous event in case of repeat,
    /// amount of rotation since start in case of a start_event.
    /// Duration is specified in ms.
    pub duration: u16,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum RelativeRotaryDirection {
    #[serde(rename = "clock_wise")]
    Clockwise,
    #[serde(rename = "counter_clock_wise")]
    CounterClockwise,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RotationReport {
    /// Last time the value of this property was updated.
    pub updated: String,
    /// Indicates which type of rotary event was received.
    pub action: RelativeRotaryAction,
    pub rotation: RelativeRotaryRotationState,
}
