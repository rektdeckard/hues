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
    #[allow(dead_code)]
    #[serde(rename = "type")]
    pub error_type: u16,
    #[allow(dead_code)]
    pub address: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterSuccessPayload {
    pub username: String,
    pub clientkey: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnregisterResponse {
    Success(String),
    Error(String),
}
