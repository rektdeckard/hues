use crate::{
    api::HueAPIError,
    command::{merge_commands, SceneCommand, SmartSceneCommand},
    service::{
        BasicStatus, Bridge, ColorFeatureBasic, EffectType, GradientMode, GradientPoint,
        GroupDimmingState, OnState, ResourceIdentifier, ResourceType,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Scene<'a> {
    bridge: &'a Bridge,
    data: SceneData,
}

impl<'a> Scene<'a> {
    pub fn new(bridge: &'a Bridge, data: SceneData) -> Self {
        Scene { bridge, data }
    }

    pub fn data(&self) -> &SceneData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn rid(&self) -> ResourceIdentifier {
        self.data.rid()
    }

    pub fn group(&self) -> ResourceIdentifier {
        self.data.group.clone()
    }

    pub fn name(&self) -> &str {
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
        self.bridge.api.put_scene(self.id(), &payload).await
    }

    pub async fn delete(&self) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        self.bridge.api.delete_scene(self.id()).await
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

    pub fn appdata(mut self, data: impl Into<String>) -> Self {
        self.metadata.appdata = Some(data.into());
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

impl SceneData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::Scene,
        }
    }
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
    /// Only accepts `rtype`: [ResourceType::PublicImage] on creation.
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SceneStatus {
    Active,
    Inactive,
    Static,
    DynamicPalette,
}

#[derive(Debug)]
pub struct SmartScene<'a> {
    bridge: &'a Bridge,
    data: SmartSceneData,
}

impl<'a> SmartScene<'a> {
    pub fn new(bridge: &'a Bridge, data: SmartSceneData) -> Self {
        SmartScene { bridge, data }
    }

    pub fn data(&self) -> &SmartSceneData {
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

    pub fn image(&self) -> Option<&ResourceIdentifier> {
        self.data.metadata.image.as_ref()
    }

    pub fn group(&self) -> ResourceIdentifier {
        self.data.group.to_owned()
    }

    pub fn builder(name: impl Into<String>, group: ResourceIdentifier) -> SmartSceneBuilder {
        SmartSceneBuilder::new(name, group)
    }

    pub async fn activate(&self) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        self.send(&[SmartSceneCommand::Enabled(true)]).await
    }

    pub async fn deactivate(&self) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        self.send(&[SmartSceneCommand::Enabled(false)]).await
    }

    pub async fn send(
        &self,
        commands: &[SmartSceneCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = merge_commands(commands);
        self.bridge.api.put_smart_scene(self.id(), &payload).await
    }

    // pub async fn delete(&self) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
    //     self.api.delete_scene(self.id()).await
    // }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SmartSceneData {
    /// Unique identifier representing a specific resource instance.
    pub id: String,
    /// Clip v1 resource identifier.
    pub id_v1: Option<String>,
    pub metadata: SceneMetadata,
    /// Group associated with this Scene. All services in the group are part of this scene.
    /// If the group is changed the scene is updated (e.g. light added/removed).
    pub group: ResourceIdentifier,
    /// Information on what is the light state for every timeslot of the day.
    pub week_timeslots: Vec<Schedule>,
    /// Duration of the transition from on one timeslot's scene to the other in ms (defaults to 60000ms).
    pub transition_duration: usize,
    /// The active time slot in execution.
    pub active_timeslot: Option<ActiveTimeslot>,
    /// The current state of the smart scene. The default state is [BasicStatus::Inactive] if no recall is provided.
    pub state: BasicStatus,
}

impl SmartSceneData {
    pub fn rid(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            rid: self.id.to_owned(),
            rtype: ResourceType::SmartScene,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schedule {
    pub timeslots: Vec<SmartSceneTimeslot>,
    pub recurrence: HashSet<Weekday>,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            timeslots: Default::default(),
            recurrence: Default::default(),
        }
    }

    pub fn on(mut self, days: &[Weekday]) -> Self {
        self.recurrence = days.iter().map(|w| w.to_owned()).collect();
        self
    }

    pub fn at(mut self, time: TimeslotStart, scene_rid: ResourceIdentifier) -> Self {
        let s = SmartSceneTimeslot {
            start_time: time,
            target: scene_rid,
        };
        self.timeslots.push(s);
        self
    }

    pub fn monday(mut self) -> Self {
        self.recurrence.insert(Weekday::Monday);
        self
    }

    pub fn tuesday(mut self) -> Self {
        self.recurrence.insert(Weekday::Tuesday);
        self
    }

    pub fn wednesday(mut self) -> Self {
        self.recurrence.insert(Weekday::Wednesday);
        self
    }

    pub fn thursday(mut self) -> Self {
        self.recurrence.insert(Weekday::Thursday);
        self
    }

    pub fn friday(mut self) -> Self {
        self.recurrence.insert(Weekday::Friday);
        self
    }

    pub fn saturday(mut self) -> Self {
        self.recurrence.insert(Weekday::Saturday);
        self
    }

    pub fn sunday(mut self) -> Self {
        self.recurrence.insert(Weekday::Sunday);
        self
    }

    pub fn build(self) -> SmartSceneCommand {
        SmartSceneCommand::Schedule(vec![Schedule {
            timeslots: self.timeslots,
            recurrence: self.recurrence,
        }])
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SmartSceneTimeslot {
    pub start_time: TimeslotStart,
    pub target: ResourceIdentifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum TimeslotStart {
    Sunset,
    Time { time: TimeslotTime },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimeslotTime {
    /// `0` to `23`
    hour: u8,
    /// `0` to `59`
    minute: u8,
    /// `0` to `59`
    second: u8,
}

impl TimeslotStart {
    pub fn time(hms: &[u8; 3]) -> TimeslotStart {
        TimeslotStart::Time {
            time: TimeslotTime {
                hour: hms[0],
                minute: hms[1],
                second: hms[2],
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActiveTimeslot {
    pub timeslot_id: usize,
    pub weekday: Weekday,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Serialize)]
pub struct SmartSceneBuilder {
    metadata: SceneMetadata,
    group: ResourceIdentifier,
    transition_duration: usize,
    week_timeslots: Vec<Schedule>,
}

impl SmartSceneBuilder {
    pub fn new(name: impl Into<String>, group: ResourceIdentifier) -> Self {
        SmartSceneBuilder {
            metadata: SceneMetadata {
                name: name.into(),
                ..Default::default()
            },
            group,
            transition_duration: 0,
            week_timeslots: Default::default(),
        }
    }

    pub fn image(mut self, image: ResourceIdentifier) -> Self {
        self.metadata.image = Some(image);
        self
    }

    pub fn transition_duration(mut self, ms: usize) -> Self {
        self.transition_duration = ms;
        self
    }

    pub fn schedule(mut self, s: Schedule) -> Self {
        self.week_timeslots.push(s);
        self
    }
}
