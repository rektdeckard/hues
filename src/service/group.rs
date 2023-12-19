use super::{
    bridge::Bridge,
    light::{AlertState, OnState, SignalType},
    resource::{ResourceIdentifier, ResourceType},
};
use crate::{
    api::HueAPIError,
    command::{merge_commands, GroupCommand},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Group<'a> {
    bridge: &'a Bridge,
    data: GroupData,
}

impl<'a> Group<'a> {
    pub fn new(bridge: &'a Bridge, data: GroupData) -> Self {
        Group { bridge, data }
    }

    pub fn data(&self) -> &GroupData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn is_on(&self) -> Option<bool> {
        self.data.on.as_ref().and_then(|on| Some(on.on))
    }

    pub async fn send(
        &self,
        commands: &[GroupCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_grouped_light(self.id(), &payload).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    pub owner: ResourceIdentifier,
    /// Joined on control & aggregated on state.
    /// `on` is true if any light in the group is on.
    pub on: Option<OnState>,
    /// Joined dimming control.
    /// `brightness` contains average brightness of group containing turned-on lights only.
    pub dimming: Option<GroupDimmingState>,
    /// Joined alert control.
    pub alert: Option<AlertState>,
    pub signaling: Option<GroupSignalingState>,
}

impl GroupData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Group,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupDimmingState {
    /// Brightness percentage.
    /// Value cannot be `0`, writing `0` changes it to lowest possible brightness.
    pub brightness: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupSignalingState {
    /// Signals that the group supports.
    pub signal_values: Option<HashSet<SignalType>>,
}
