use crate::api::{self, HueAPI, HueAPIError, Version, V2};
use crate::command::{CommandBuilder, CommandType};
use crate::group::Group;
use crate::light::Light;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

#[derive(Debug)]
pub enum BridgeDiscoveryError {
    NotFound,
    MDNSUnavailable,
}

#[derive(Debug)]
pub enum BridgeBuildError {
    NoIp,
    NoAppKey,
}

#[derive(Debug)]
pub enum BridgeUserError {
    UnableToCreate,
}

/// Core structure representing a Hue Bridge device interface.
pub struct Bridge<'a> {
    pub(crate) api: Box<V2>,
    lights: HashMap<String, Light<'a>>,
}

impl<'a> Bridge<'a> {
    pub fn new(addr: impl Into<IpAddr>, app_key: impl Into<String>) -> Self {
        let api = V2::new(addr.into(), app_key.into());
        Bridge {
            api: Box::new(api),
            lights: HashMap::new(),
        }
    }

    pub async fn discover() -> Result<BridgeBuilder<'a>, BridgeDiscoveryError> {
        BridgeBuilder::discover().await
    }

    pub async fn create_app(
        mut self,
        app_name: impl Into<String>,
        instance_name: impl Into<String>,
    ) -> Result<String, HueAPIError> {
        self.api.create_app(app_name, instance_name).await
    }

    pub async fn lights(&'a mut self) -> Result<&'a HashMap<String, Light>, api::HueAPIError> {
        let res = self.api.get_lights().await?;
        self.lights.extend(
            res.data
                .into_iter()
                .map(|light| (light.id.clone(), Light::new(&self.api, light))),
        );
        Ok(&self.lights)
    }

    pub fn command(&self) -> CommandBuilder {
        CommandBuilder::new(&self)
    }
}

/// Builder structure representing a [Bridge] that is not yet fully configured.
pub struct BridgeBuilder<'a> {
    addr: Option<IpAddr>,
    app_key: Option<String>,
    heartbeat: Option<Duration>,
    version: Version,
    p: &'a PhantomData<Bridge<'a>>,
}

impl<'a> Default for BridgeBuilder<'a> {
    fn default() -> Self {
        BridgeBuilder {
            addr: None,
            app_key: None,
            heartbeat: None,
            version: Default::default(),
            p: &PhantomData,
        }
    }
}

impl<'a> BridgeBuilder<'a> {
    pub fn new() -> Self {
        BridgeBuilder::default()
    }

    async fn discover_http() -> Result<Self, BridgeDiscoveryError> {
        todo!()
    }

    #[cfg(feature = "mdns")]
    async fn discover_mdns() -> Result<Self, BridgeDiscoveryError> {
        const SERVICE_NAME: &'static str = "_hue._tcp.local";
        use futures_util::{pin_mut, stream::StreamExt};
        let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))
            .unwrap()
            .listen();
        pin_mut!(stream);

        if let Some(Ok(response)) = stream.next().await {
            for rec in response.answers {
                match rec.kind {
                    mdns::RecordKind::A(addr) => {
                        return Ok(BridgeBuilder {
                            addr: Some(addr.into()),
                            ..Default::default()
                        })
                    }
                    _ => {}
                }
            }
            Err(BridgeDiscoveryError::NotFound)
        } else {
            Err(BridgeDiscoveryError::MDNSUnavailable)
        }
    }

    pub async fn discover() -> Result<Self, BridgeDiscoveryError> {
        #[cfg(feature = "mdns")]
        if let Ok(bridge) = BridgeBuilder::discover_mdns().await {
            return Ok(bridge);
        }
        BridgeBuilder::discover_http().await
    }

    pub fn app_key(mut self, key: &str) -> Self {
        self.app_key = Some(key.into());
        self
    }

    pub fn version(mut self, v: Version) -> Self {
        self.version = v;
        self
    }

    pub fn heartbeat(mut self, interval: Duration) -> Self {
        todo!()
    }

    pub fn build(self) -> Bridge<'a> {
        let api = if self.version == Version::V2 {
            V2::new(
                self.addr.unwrap_or([0u8, 0, 0, 0].into()),
                self.app_key.unwrap_or_default(),
            )
        } else {
            todo!()
        };

        Bridge {
            api: Box::new(api),
            lights: HashMap::new(),
        }
    }
}
