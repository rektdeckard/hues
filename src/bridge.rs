use crate::api;
use crate::light::Light;
use std::{net::Ipv4Addr, time::Duration};

pub struct Bridge {
    addr: Ipv4Addr,
}

impl Bridge {
    async fn discover_http() -> Result<Self, ()> {
        todo!()
    }

    #[cfg(feature = "mdns")]
    async fn discover_mdns() -> Result<Self, ()> {
        const SERVICE_NAME: &'static str = "_hue._tcp.local";
        use futures_util::{pin_mut, stream::StreamExt};
        use mdns::{Error, Record, RecordKind};
        let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))
            .unwrap()
            .listen();
        pin_mut!(stream);

        if let Some(Ok(response)) = stream.next().await {
            for rec in response.answers {
                match rec.kind {
                    RecordKind::A(addr) => return Ok(Bridge { addr }),
                    _ => {}
                }
            }
            Err(())
        } else {
            panic!("Didn't find anything!")
        }
    }

    pub async fn discover() -> Result<Self, ()> {
        #[cfg(feature = "mdns")]
        if let Ok(bridge) = Bridge::discover_mdns().await {
            return Ok(bridge);
        }
        Bridge::discover_http().await
    }

    pub fn with_addr(addr: Ipv4Addr) -> Self {
        Bridge { addr }
    }

    pub async fn heartbeat(mut self, interval: Duration) -> Self {
        self
    }
}
