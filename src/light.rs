use crate::{
    api::{HueAPI, HueAPIError, LightGet, V2},
    LightCommand,
};

pub struct Light<'a> {
    api: &'a V2,
    pub data: LightGet,
}

impl<'a> Light<'a> {
    pub fn new(api: &'a V2, data: LightGet) -> Self {
        Light { api, data }
    }

    pub async fn identify(&self) -> Result<(), HueAPIError> {
        self.api.identify_light(&self.data.id).await
    }

    pub async fn modify(&self, commands: &[LightCommand]) -> Result<(), HueAPIError> {
        self.api.modify_light(&self.data.id).await
    }

    pub fn command(&self) -> LightCommandBuilder {
        LightCommandBuilder::new(&self.api)
    }
}

struct LightCommandBuilder<'a> {
    api: &'a V2,
    commands: Vec<LightCommand>,
}

impl<'a> LightCommandBuilder<'a> {
    fn new(api: &'a V2) -> Self {
        LightCommandBuilder {
            api,
            commands: vec![],
        }
    }

    pub fn power(mut self, on: bool) -> Self {
        self.commands.push(LightCommand::Power(on));
        self
    }

    pub fn on(mut self) -> Self {
        self.power(true)
    }

    pub fn off(mut self) -> Self {
        self.power(false)
    }

    pub fn identify(mut self) -> Self {
        self.commands.push(LightCommand::Identify);
        self
    }

    pub async fn send(&self) -> Result<()< HueAPIError> {

    }
}
