use crate::{
    api::{HueAPIError, LightGet, ResourceIdentifier, V2},
    Command, LightCommand,
};

pub struct Light<'a> {
    api: &'a V2,
    pub data: LightGet,
}

impl<'a> Light<'a> {
    pub fn new(api: &'a V2, data: LightGet) -> Self {
        Light { api, data }
    }

    pub async fn identify(&self) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        self.api.identify_light(&self.data.id).await
    }

    pub async fn send(
        &self,
        commands: &[LightCommand],
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = Command::merge(commands);
        self.api.update_light(&self.data.id, &payload).await
    }

    pub fn command(&self) -> LightCommandBuilder {
        LightCommandBuilder::new(&self.api, &self.data.id)
    }
}

pub struct LightCommandBuilder<'a> {
    api: &'a V2,
    id: String,
    commands: Vec<LightCommand>,
}

impl<'a> LightCommandBuilder<'a> {
    fn new(api: &'a V2, id: impl Into<String>) -> Self {
        LightCommandBuilder {
            api,
            id: id.into(),
            commands: vec![],
        }
    }

    pub fn power(mut self, on: bool) -> Self {
        self.commands.push(LightCommand::Power(on));
        self
    }

    pub fn on(self) -> Self {
        self.power(true)
    }

    pub fn off(self) -> Self {
        self.power(false)
    }

    pub fn identify(mut self) -> Self {
        self.commands.push(LightCommand::Identify);
        self
    }

    pub fn alert(mut self) -> Self {
        self.commands
            .push(LightCommand::Alert(crate::api::AlertEffectType::Breathe));
        self
    }

    pub async fn send(&self) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let payload = Command::merge(&self.commands);
        self.api.update_light(&self.id, &payload).await
    }
}
