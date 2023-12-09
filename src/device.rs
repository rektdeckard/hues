use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Device {
    #[serde(rename = "light")]
    Light,
}
