use super::{
    group::GroupDimmingState,
    light::EffectType,
    light::{ColorFeatureBasic, GradientMode, GradientPoint, OnState},
    resource::{ResourceIdentifier, ResourceType},
};
use crate::{
    api::{BridgeClient, HueAPIError},
    command::{merge_commands, SceneCommand},
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Scene<'a> {
    api: &'a BridgeClient,
    data: SceneData,
}

impl<'a> Scene<'a> {
    pub fn new(api: &'a BridgeClient, data: SceneData) -> Self {
        Scene { api, data }
    }

    pub fn data(&self) -> &SceneData {
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub fn name(&self) -> &String {
        &self.data.metadata.name
    }

    pub fn image(&self) -> Option<&ResourceIdentifier> {
        self.data.metadata.image.as_ref()
    }

    pub fn status(&self) -> SceneStatus {
        self.data.status.active
    }

    pub fn builder(name: impl Into<String>, group: ResourceIdentifier) -> SceneBuilder {
        SceneBuilder::new(name, group)
    }

    pub async fn recall(&self) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        self.send(&[SceneCommand::Recall {
            action: Some(SceneStatus::Active),
            duration: None,
            dimming: None,
        }])
        .await
    }

    pub async fn send(
        &self,
        commands: &[SceneCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.api.put_scene(self.id(), &payload).await
    }

    pub async fn delete(&self) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        self.api.delete_scene(self.id()).await
    }
}

#[derive(Serialize)]
pub struct SceneBuilder {
    actions: Vec<SceneAction>,
    metadata: SceneMetadata,
    group: ResourceIdentifier,
    palette: Option<ScenePalette>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_dynamic: Option<bool>,
}

impl SceneBuilder {
    pub fn new(name: impl Into<String>, group: ResourceIdentifier) -> Self {
        SceneBuilder {
            actions: vec![],
            metadata: SceneMetadata {
                name: name.into(),
                ..Default::default()
            },
            palette: None,
            group,
            speed: None,
            auto_dynamic: None,
        }
    }

    pub fn actions(mut self, actions: Vec<SceneAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn image(mut self, image: ResourceIdentifier) -> Self {
        self.metadata.image = Some(image);
        self
    }

    pub fn palette(mut self, palette: ScenePalette) -> Self {
        self.palette = Some(palette);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = Some(speed);
        self
    }

    pub fn auto_dynamic(mut self, auto_dynamic: bool) -> Self {
        self.auto_dynamic = Some(auto_dynamic);
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SceneData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    /// List of actions to be executed synchronously on recall.
    pub actions: Vec<SceneAction>,
    pub metadata: SceneMetadata,
    /// Group associated with this Scene. All services in the group are part of this scene.
    /// If the group is changed the scene is updated (e.g. light added/removed).
    pub group: ResourceIdentifier,
    /// Group of colors that describe the palette of colors to be used when playing dynamics.
    pub palette: Option<ScenePalette>,
    /// Speed of dynamic palette for this scene.
    pub speed: f32,
    /// Indicates whether to automatically start the scene dynamically on active recall.
    pub auto_dynamic: bool,
    pub status: SceneStatusState,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SceneAction {
    /// The identifier of the light to execute the action on.
    pub target: ResourceIdentifier,
    /// The action to be executed on recall.
    pub action: LightAction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LightAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<OnState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimming: Option<GroupDimmingState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorFeatureBasic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<SceneColorTempState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<SceneGradientState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<SceneEffectState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<SceneDynamics>,
}

impl Default for LightAction {
    fn default() -> Self {
        LightAction {
            on: Some(OnState { on: true }),
            dimming: None,
            color: None,
            color_temperature: None,
            gradient: None,
            effects: None,
            dynamics: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SceneColorTempState {
    /// Color temperature in mirek or `None` when the light color is not in the ct spectrum.
    pub mirek: Option<u16>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SceneGradientState {
    /// Collection of gradients points.
    /// For control of the gradient points through a PUT a minimum of 2 points need to be provided.
    pub points: Vec<GradientPoint>,
    /// Mode in which the points are currently being deployed.
    /// If not provided during PUT/POST it will be defaulted to [GradientMode::InterpolatedPalette].
    pub mode: GradientMode,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SceneEffectState {
    pub effect: Option<EffectType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SceneDynamics {
    /// Duration of a light transition or timed effects in ms.
    pub duration: Option<usize>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SceneMetadata {
    /// Human readable name of a resource.
    pub name: String,
    /// Reference with unique identifier for the image representing the scene.
    /// Only accepts `rtype`: [ResourceType::PublicImage](crate::resource::ResourceType::PublicImage) on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ResourceIdentifier>,
    /// Application specific data. Free format string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appdata: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ScenePalette {
    pub color: Vec<ScenePaletteColor>,
    pub dimming: Vec<GroupDimmingState>,
    pub color_temperature: Vec<ScenePaletteColorTempState>,
    pub effects: Vec<SceneEffectState>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScenePaletteColor {
    pub color: ColorFeatureBasic,
    pub dimming: GroupDimmingState,
}

impl ScenePaletteColor {
    pub fn xyb(x: f32, y: f32, b: f32) -> Self {
        ScenePaletteColor {
            color: ColorFeatureBasic::xy(x, y),
            dimming: GroupDimmingState { brightness: b },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScenePaletteColorTempState {
    pub color_temperature: SceneColorTempState,
    pub dimming: GroupDimmingState,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SceneStatusState {
    pub active: SceneStatus,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneStatus {
    Active,
    Inactive,
    Static,
    DynamicPalette,
}
