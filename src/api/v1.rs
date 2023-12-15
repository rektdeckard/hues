use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum RegisterResponse {
    Success { success: RegisterSuccessPayload },
    Error { error: RegisterErrorPayload },
}

#[derive(Debug, Deserialize)]
pub struct RegisterErrorPayload {
    #[serde(rename = "type")]
    pub error_type: u16,
    pub address: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterSuccessPayload {
    pub username: String,
    pub clientkey: String,
}
