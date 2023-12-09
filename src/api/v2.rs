use super::v1::RegisterResponse;
use super::{HueAPI, HueAPIError, HueAPIV2Response, LightGet};
use crate::command::CommandType;
use crate::light::Light;
use reqwest::Client;
use serde_json::json;
use std::net::IpAddr;

const PREFIX: &'static str = "/clip/v2";

pub struct V2 {
    addr: IpAddr,
    app_key: String,
    client: Client,
}

impl V2 {
    pub fn new(addr: impl Into<IpAddr>, app_key: impl Into<String>) -> Self {
        V2 {
            addr: addr.into(),
            app_key: app_key.into(),
            client: Client::builder()
                // FIXME: why cert :()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        }
    }

    pub fn addr(&self) -> String {
        self.addr.to_string().clone()
    }

    pub async fn create_app(
        &mut self,
        app_name: impl Into<String>,
        instance_name: impl Into<String>,
    ) -> Result<String, HueAPIError> {
        match self
            .client
            .post(self.api_v1_url())
            .json(&json!({
               "devicetype": format!("{}#{}", app_name.into(), instance_name.into()),
               "generateclientkey": true
            }))
            .send()
            .await
        {
            Ok(res) => match res.json::<Vec<super::v1::RegisterResponse>>().await {
                Ok(successes_or_errors) => match successes_or_errors.get(1).unwrap() {
                    RegisterResponse::Success { success } => Ok(success.username.clone()),
                    RegisterResponse::Error { error } => {
                        Err(HueAPIError::HueBridgeError(error.description.clone()))
                    }
                },
                Err(_) => Err(HueAPIError::BadDeserialize),
            },
            Err(_) => Err(HueAPIError::BadRequest),
        }
    }

    fn api_url(&self) -> String {
        format!("https://{}{}", &self.addr, PREFIX)
    }

    fn api_v1_url(&self) -> String {
        format!("https://{}/api", &self.addr)
    }
}

impl HueAPI for V2 {
    async fn identify_light(&self, id: impl Into<String>) -> Result<(), HueAPIError> {
        let id = id.into();
        dbg!(self.api_url() + "/resource/light/" + &id);
        match self
            .client
            .put(self.api_url() + "/resource/light/" + &id)
            .header("hue-application-key", &self.app_key)
            .json(&json!({ "identify": { "action": "identify" } }))
            // .json(&json!({ "on": { "on": true } }))
            .send()
            .await
        {
            Ok(response) => {
                dbg!(response.text().await);
                Ok(())
            }
            Err(e) => Err(HueAPIError::BadRequest),
        }
    }

    async fn modify_light(&self, id: impl Into<String>, commands: &[CommandType::]) -> Result<(), HueAPIError> {
        let id = id.into();
        dbg!(self.api_url() + "/resource/light/" + &id);
        match self
            .client
            .put(self.api_url() + "/resource/light/" + &id)
            .header("hue-application-key", &self.app_key)
            .json(&json!({ "identify": { "action": "identify" } }))
            // .json(&json!({ "on": { "on": true } }))
            .send()
            .await
        {
            Ok(response) => {
                dbg!(response.text().await);
                Ok(())
            }
            Err(e) => Err(HueAPIError::BadRequest),
        }
    }

    async fn get_lights(&self) -> Result<HueAPIV2Response<Vec<LightGet>>, HueAPIError> {
        dbg!(self.api_url() + "/resource/light");
        match self
            .client
            .get(self.api_url() + "/resource/light")
            .header("hue-application-key", &self.app_key)
            .send()
            .await
        {
            Ok(response) => match response.json::<HueAPIV2Response<Vec<LightGet>>>().await {
                Ok(d) => {
                    // dbg!(&d);
                    return Ok(d);
                }
                Err(e) => {
                    panic!("{}", e)
                }
            },
            Err(e) => {
                panic!("{}", e);
                Err(HueAPIError::BadRequest)
            }
        }
    }
}
