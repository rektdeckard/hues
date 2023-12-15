mod v1;
mod v2;

use serde::Deserialize;

pub(crate) use v2::BridgeClient;

#[derive(Debug, Deserialize)]
pub struct HueAPIResponse<D> {
    pub(crate) errors: Vec<HueAPIErrorMessage>,
    pub(crate) data: Option<D>,
}

#[derive(Debug, Deserialize)]
pub struct HueAPIErrorMessage {
    /// A human-readable explanation specific to this occurrence of the problem.
    pub description: String,
}

#[derive(Debug, PartialEq)]
pub enum HueAPIError {
    BadRequest,
    BadResponse,
    BadDeserialize,
    NotFound,
    HueBridgeError(String),
}

#[derive(Default, PartialEq)]
pub enum Version {
    V1,
    #[default]
    V2,
}
