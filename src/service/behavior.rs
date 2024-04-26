use crate::{
    api::HueAPIError,
    command::{merge_commands, BehaviorInstanceCommand},
    service::{BasicMetadata, Bridge, ResourceIdentifier, ResourceType},
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BehaviorScript {
    data: BehaviorScriptData,
}

impl BehaviorScript {
    pub fn new(data: BehaviorScriptData) -> Self {
        BehaviorScript { data }
    }

    pub fn data(&self) -> &BehaviorScriptData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn name(&self) -> Option<&str> {
        self.data.metadata.name.as_deref()
    }

    pub fn category(&self) -> &BehaviorScriptType {
        &self.data.metadata.category
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct BehaviorScriptData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Short description of script..
    pub description: String,
    /// JSON schema object used for validating ScriptInstance.configuration property.
    pub configuration_schema: BehaviorSchema,
    /// JSON schema object used for validating ScriptInstance.configuration property.
    pub trigger_schema: BehaviorSchema,
    /// JSON schema object used for validating ScriptInstance.configuration property.
    pub state_schema: BehaviorSchema,
    /// Version of script.
    pub version: String,
    pub metadata: BehaviorScriptMetadata,
    /// Features that the script supports.
    pub supported_features: Option<Vec<String>>,
    /// Max number of script instances.
    pub max_number_instances: Option<u8>,
}

impl BehaviorScriptData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::BehaviorScript,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct BehaviorScriptMetadata {
    /// Human readable name of a resource.
    pub name: Option<String>,
    pub category: BehaviorScriptType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum BehaviorSchema {
    Ref(SchemaRef),
    Lit(serde_json::Value),
}

#[derive(Clone, Debug, Deserialize)]
pub struct SchemaRef {
    #[serde(rename = "$ref")]
    pub sref: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorScriptType {
    Automation,
    Entertainment,
    Accessory,
}

// #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
// #[serde(rename_all = "snake_case")]
// pub enum BehaviorFeature {
//     StyleSunrise,
//     Intensity,
//     #[serde(other)]
//     Other,
// }

#[derive(Debug)]
pub struct BehaviorInstance<'a> {
    bridge: &'a Bridge,
    data: BehaviorInstanceData,
}

impl<'a> BehaviorInstance<'a> {
    pub fn new(bridge: &'a Bridge, data: BehaviorInstanceData) -> Self {
        BehaviorInstance { bridge, data }
    }

    pub fn data(&self) -> &BehaviorInstanceData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn is_enabled(&self) -> bool {
        self.data.enabled
    }

    pub fn builder(
        script_id: impl Into<String>,
        configuration: serde_json::Value,
    ) -> BehaviorInstanceBuilder {
        BehaviorInstanceBuilder::new(script_id, configuration)
    }

    pub async fn send(
        &self,
        commands: &[BehaviorInstanceCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge
            .api
            .put_behavior_instance(self.id(), &payload)
            .await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct BehaviorInstanceData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// Identifier to ScriptDefinition.
    pub script_id: String,
    /// Indicated whether a scripts is enabled.
    pub enabled: bool,
    /// Script instance state. This read-only property is according to ScriptDefinition.state_schema JSON schema.
    pub state: Option<serde_json::Value>,
    /// Script instance state. This read-only property is according to ScriptDefinition.state_schema JSON schema.
    pub configuration: serde_json::Value,
    /// Represents all resources which this instance depends on.
    pub dependees: Vec<ResourceDependee>,
    /// Script status. If the script is in the errored state then check errors for more details about the error.
    pub status: BehaviorInstanceStatus,
    /// Last error happened while executing the script.
    pub last_error: Option<String>,
    pub metadata: BasicMetadata,
    /// Clip v1 resource identifier.
    pub migrated_from: Option<String>,
}

impl BehaviorInstanceData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::BehaviorScript,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResourceDependee {
    target: ResourceIdentifier,
    level: ResourceDependeeImportance,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceDependeeImportance {
    Critical,
    NonCritical,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorInstanceStatus {
    Initializing,
    Running,
    Disabled,
    Errored,
}

#[derive(Serialize)]
pub struct BehaviorInstanceBuilder {
    script_id: String,
    enabled: bool,
    configuration: serde_json::Value,
    metadata: BasicMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    migrated_from: Option<String>,
}

impl BehaviorInstanceBuilder {
    pub fn new(script_id: impl Into<String>, configuration: serde_json::Value) -> Self {
        BehaviorInstanceBuilder {
            script_id: script_id.into(),
            enabled: false,
            configuration,
            metadata: BasicMetadata { name: None },
            migrated_from: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.metadata.name = Some(name.into());
        self
    }

    pub fn migrated_from(mut self, id: impl Into<String>) -> Self {
        self.migrated_from = Some(id.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}
