use super::{
    light::{AlertState, OnState, SignalType},
    resource::{ResourceIdentifier, ResourceType},
};
use crate::{
    api::{BridgeClient, HueAPIError},
    command::{merge_commands, GroupCommand},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Group<'a> {
    api: &'a BridgeClient,
    data: GroupData,
}

impl<'a> Group<'a> {
    pub fn new(api: &'a BridgeClient, data: GroupData) -> Self {
        Group { api, data }
    }

    pub fn data(&self) -> &GroupData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub fn is_on(&self) -> Option<bool> {
        self.data.on.as_ref().and_then(|on| Some(on.on))
    }

    pub async fn send(
        &self,
        commands: &[GroupCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_grouped_light(self.id(), &payload).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GroupData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupDimmingState {
    /// Brightness percentage.
    /// Value cannot be `0`, writing `0` changes it to lowest possible brightness.
    pub brightness: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GroupSignalingState {
    /// Signals that the group supports.
    pub signal_values: Option<HashSet<SignalType>>,
}
