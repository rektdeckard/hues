use super::v1::RegisterResponse;
use super::{HueAPIError, HueAPIResponse, LightGet, ResourceIdentifier};
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
    pub(crate) fn new(addr: impl Into<IpAddr>, app_key: impl Into<String>) -> Self {
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

    pub(crate) fn addr(&self) -> String {
        self.addr.to_string().clone()
    }

    pub(crate) async fn create_app(
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
                _ => Err(HueAPIError::BadDeserialize),
            },
            _ => Err(HueAPIError::BadRequest),
        }
    }

    fn api_url(&self) -> String {
        format!("https://{}{}", &self.addr, PREFIX)
    }

    fn api_v1_url(&self) -> String {
        format!("https://{}/api", &self.addr)
    }

    pub(crate) async fn identify_light(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let id = id.into();
        match self
            .client
            .put(self.api_url() + "/resource/light/" + &id)
            .header("hue-application-key", &self.app_key)
            .json(&json!({ "identify": { "action": "identify" } }))
            // .json(&json!({ "on": { "on": true } }))
            .send()
            .await
        {
            Ok(res) => {
                return match res.json::<HueAPIResponse<Vec<ResourceIdentifier>>>().await {
                    Ok(res) => {
                        if res.errors.is_empty() {
                            Ok(res.data)
                        } else {
                            Err(HueAPIError::HueBridgeError(
                                res.errors[0].description.clone(),
                            ))
                        }
                    }
                    _ => Err(HueAPIError::BadDeserialize),
                }
            }
            _ => Err(HueAPIError::BadRequest),
        }
    }

    pub(crate) async fn update_light(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let id = id.into();
        match self
            .client
            .put(self.api_url() + "/resource/light/" + &id)
            .header("hue-application-key", &self.app_key)
            .json(payload)
            // .json(&json!({ "on": { "on": true } }))
            .send()
            .await
        {
            Ok(res) => {
                return match res.json::<HueAPIResponse<Vec<ResourceIdentifier>>>().await {
                    Ok(res) => {
                        if res.errors.is_empty() {
                            Ok(res.data)
                        } else {
                            Err(HueAPIError::HueBridgeError(
                                res.errors[0].description.clone(),
                            ))
                        }
                    }
                    _ => Err(HueAPIError::BadDeserialize),
                }
            }
            _ => Err(HueAPIError::BadRequest),
        }
    }

    pub(crate) async fn get_lights(&self) -> Result<HueAPIResponse<Vec<LightGet>>, HueAPIError> {
        match self
            .client
            .get(self.api_url() + "/resource/light")
            .header("hue-application-key", &self.app_key)
            .send()
            .await
        {
            Ok(res) => res
                .json::<HueAPIResponse<Vec<LightGet>>>()
                .await
                .map_err(|_| HueAPIError::BadDeserialize),
            _ => Err(HueAPIError::BadRequest),
        }
    }
}
