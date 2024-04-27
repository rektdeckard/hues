mod v1;
mod v2;

use serde::Deserialize;

pub(crate) use v2::BridgeClient;

#[derive(Debug, Deserialize)]
pub(crate) struct HueAPIResponse<D> {
    pub errors: Vec<HueAPIErrorMessage>,
    pub data: Option<D>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct HueAPIErrorMessage {
    /// A human-readable explanation specific to this occurrence of the problem
    pub description: String,
}

/// Possible errors related to communication with the Hue Bridge.
#[derive(Debug, PartialEq)]
pub enum HueAPIError {
    BadRequest,
    BadResponse,
    BadDeserialize,
    NotFound,
    HueBridgeError(String),
    ServerSentEvent,
    Streaming,
}

/// The protol used by the Hue Bridge, currently only [`Version::V2`] is supported.
#[derive(Default, PartialEq)]
pub enum Version {
    V1,
    #[default]
    V2,
}
