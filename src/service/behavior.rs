use super::resource::{ResourceIdentifier, ResourceType};
use serde::Deserialize;

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
