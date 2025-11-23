use crate::{
    api::{v1::RegisterResponse, HueAPIError, HueAPIResponse},
    service::{
        BehaviorInstanceData, BehaviorScriptData, BridgeData, ButtonData, ContactData, DeviceData,
        DevicePowerData, EntertainmentConfigurationData, EntertainmentData, GeofenceClientData,
        GeolocationData, GroupData, HomeData, HomeKitData, LightData, LightLevelData, MatterData,
        MatterFabricData, MotionData, RelativeRotaryData, Resource, ResourceIdentifier, SceneData,
        SmartSceneData, TamperData, TemperatureData, ZGPConnectivityData, ZigbeeConnectivityData,
        ZigbeeDeviceDiscoveryData, ZoneData,
    },
};

use reqwest::{Certificate, Client as ReqwestClient, IntoUrl, Method};
use std::net::IpAddr;

#[cfg(feature = "sse")]
use reqwest_eventsource::EventSource;

#[cfg(feature = "streaming")]
use rustls::{pki_types::CertificateDer, RootCertStore};

const V2_PREFIX: &'static str = "/clip/v2";
#[allow(dead_code)]
const UDP_PORT: usize = 2100;

#[derive(Clone, Debug)]
pub struct BridgeClient {
    addr: IpAddr,
    app_key: String,
    client_key: Option<String>,
    client: ReqwestClient,
    #[cfg(feature = "streaming")]
    root_store: RootCertStore,
}

#[allow(dead_code)]
impl BridgeClient {
    pub(crate) fn new(addr: impl Into<IpAddr>, app_key: impl Into<String>) -> Self {
        BridgeClient {
            addr: addr.into(),
            app_key: app_key.into(),
            client_key: None,
            client: ReqwestClient::builder()
                .add_root_certificate(
                    Certificate::from_pem(include_bytes!("../../hue.pem")).unwrap(),
                )
                // FIXME: why cert :(
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
            #[cfg(feature = "streaming")]
            root_store: {
                let cert = CertificateDer::from(include_bytes!("../../hue.der").to_vec());
                let mut root_store = rustls::RootCertStore::empty();
                root_store.add(cert).unwrap();
                root_store
            },
        }
    }

    #[cfg(feature = "streaming")]
    pub(crate) fn new_with_streaming(
        addr: impl Into<IpAddr>,
        app_key: impl Into<String>,
        client_key: impl Into<String>,
    ) -> Self {
        BridgeClient {
            addr: addr.into(),
            app_key: app_key.into(),
            client_key: Some(client_key.into()),
            client: ReqwestClient::builder()
                .add_root_certificate(
                    Certificate::from_pem(include_bytes!("../../hue.pem")).unwrap(),
                )
                // FIXME: why cert :(
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
            #[cfg(feature = "streaming")]
            root_store: {
                let cert = CertificateDer::from(include_bytes!("../../hue.der").to_vec());
                let mut root_store = rustls::RootCertStore::empty();
                root_store.add(cert).unwrap();
                root_store
            },
        }
    }

    pub fn addr(&self) -> &IpAddr {
        &self.addr
    }

    pub fn app_key(&self) -> &str {
        &self.app_key
    }

    #[allow(dead_code)]
    pub fn client_key(&self) -> Option<&str> {
        self.client_key.as_deref()
    }

    fn api_url(&self) -> String {
        format!("https://{}{}", &self.addr, V2_PREFIX)
    }

    fn api_v1_url(&self) -> String {
        format!("https://{}/api", &self.addr)
    }

    #[allow(dead_code)]
    fn auth_url(&self) -> String {
        format!("https://{}/auth/v1", &self.addr)
    }

    fn event_stream_url(&self) -> String {
        format!("https://{}/eventstream{}", &self.addr, V2_PREFIX)
    }

    #[allow(dead_code)]
    pub(crate) fn entertainment_url(&self) -> String {
        format!("{}:{}", &self.addr, UDP_PORT)
    }

    async fn make_request<Body: serde::Serialize, Return>(
        &self,
        url: impl IntoUrl,
        method: Method,
        body: Option<Body>,
    ) -> Result<Return, HueAPIError>
    where
        Return: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        match self
            .client
            .request(method, url)
            .header("hue-application-key", &self.app_key)
            .json(&body)
            .send()
            .await
        {
            Ok(res) => match res.json::<HueAPIResponse<Return>>().await {
                Ok(res) => {
                    if res.errors.is_empty() && res.data.is_some() {
                        Ok(res.data.unwrap())
                    } else {
                        Err(HueAPIError::HueBridgeError(serde_json::json!(res
                            .errors
                            .into_iter()
                            .map(|e| serde_json::from_str::<serde_json::Value>(
                                e.description.as_str().unwrap(),
                            )
                            .unwrap())
                            .collect::<Vec<_>>())))
                    }
                }
                Err(e) => {
                    log::error!("{e}");
                    Err(HueAPIError::BadDeserialize)
                }
            },
            _ => Err(HueAPIError::BadRequest),
        }
    }

    pub(crate) async fn create_app(
        &mut self,
        app_name: impl Into<String>,
        instance_name: impl Into<String>,
    ) -> Result<&str, HueAPIError> {
        match self
            .client
            .post(self.api_v1_url())
            .json(&serde_json::json!({
               "devicetype": format!("{}#{}", app_name.into(), instance_name.into()),
               "generateclientkey": true
            }))
            .send()
            .await
        {
            Ok(res) => match res.json::<Vec<super::v1::RegisterResponse>>().await {
                Ok(successes_or_errors) => {
                    // TODO Fix the never_loop in https://github.com/rektdeckard/hues/issues/12
                    #[warn(clippy::never_loop)]
                    for item in successes_or_errors {
                        match item {
                            RegisterResponse::Success { success } => {
                                self.app_key = success.username;
                                self.client_key = Some(success.clientkey);
                                return Ok(&self.app_key);
                            }
                            RegisterResponse::Error { error } => {
                                return Err(HueAPIError::HueBridgeError(serde_json::Value::from(
                                    error.description,
                                )))
                            }
                        }
                    }
                    return Err(HueAPIError::HueBridgeError("received no events".into()));
                }
                _ => Err(HueAPIError::BadDeserialize),
            },
            _ => Err(HueAPIError::BadRequest),
        }
    }

    pub(crate) async fn delete_app(&self, app_key: impl Into<String>) -> Result<(), HueAPIError> {
        match self
            .client
            .delete(
                self.api_v1_url() + "/" + &self.app_key + "/config/whitelist/" + &app_key.into(),
            )
            .send()
            .await
        {
            Ok(res) => match res.json::<Vec<super::v1::UnregisterResponse>>().await {
                Ok(successes_or_errors) => match successes_or_errors.into_iter().next().unwrap() {
                    super::v1::UnregisterResponse::Success(_message) => Ok(()),
                    super::v1::UnregisterResponse::Error(message) => Err(
                        HueAPIError::HueBridgeError(serde_json::Value::from(message)),
                    ),
                },
                _ => Err(HueAPIError::BadDeserialize),
            },
            _ => Err(HueAPIError::BadRequest),
        }
    }

    #[cfg(feature = "streaming")]
    pub(crate) async fn open_stream(&self, ent_id: impl Into<String>) -> Result<(), HueAPIError> {
        use std::sync::Arc;
        use tokio::net::UdpSocket;
        use webrtc_dtls::cipher_suite::CipherSuiteId;
        use webrtc_dtls::config::{Config, ExtendedMasterSecretType};
        use webrtc_dtls::conn::DTLSConn;
        use webrtc_dtls::crypto::Certificate;
        use webrtc_dtls::Error;
        use webrtc_util::Conn;

        let id: String = ent_id.into();

        match self
            .client
            .request(Method::GET, self.auth_url())
            .header("hue-application-key", &self.app_key)
            .send()
            .await
        {
            Ok(res) => match res.headers().get("hue-application-id") {
                Some(app_id) => {
                    let hue_app_id = app_id.to_str().unwrap().to_owned();

                    dbg!(self
                        .put_entertainment_configuration(
                            id.clone(),
                            &serde_json::json!({ "action": "start" })
                        )
                        .await
                        .unwrap());

                    let conn = Arc::new(UdpSocket::bind("0.0.0.0:0").await.unwrap());
                    conn.connect(self.entertainment_url()).await.unwrap();
                    println!("connecting..");

                    let client_key = self.client_key.clone().unwrap();
                    let config = Config {
                        insecure_skip_verify: true,
                        psk: Some(Arc::new(move |hint: &[u8]| -> Result<Vec<u8>, Error> {
                            println!("Client's hint: {}", String::from_utf8(hint.to_vec())?);
                            Ok(client_key.as_bytes().to_vec())
                        })),
                        // certificates: vec![
                        //     Certificate::from_pem(include_str!("../../hue.pem")).unwrap()
                        // ],
                        psk_identity_hint: Some(hue_app_id.into()),
                        cipher_suites: vec![CipherSuiteId::Tls_Psk_With_Aes_128_Gcm_Sha256],
                        extended_master_secret: ExtendedMasterSecretType::Require,
                        ..Default::default()
                    };

                    std::thread::sleep(std::time::Duration::from_millis(2000));

                    let dtls_conn: Arc<dyn Conn + Send + Sync> =
                        Arc::new(DTLSConn::new(conn, config, true, None).await.unwrap());

                    let mut bytes: Vec<u8> = vec![];
                    bytes.extend("HueStream".as_bytes()); // protocol
                    bytes.extend(&[0x02, 0x00]); // version 2.0
                    bytes.push(0x07); // sequence 7
                    bytes.extend(&[0x00, 0x00]); // reserved
                    bytes.push(0x00); // color mode RGB
                    bytes.push(0x00); // reserved
                    bytes.extend(id.as_bytes()); // entertainment configuration id

                    bytes.push(0x00); // channel 0
                    bytes.extend(&[0xff, 0xff, 0x00, 0x00, 0x00, 0x00]); // red

                    bytes.push(0x00); // channel 1
                    bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0xff, 0xff]); // red

                    println!("{:x?}", &bytes);

                    let res = dtls_conn.send(&bytes).await.unwrap();

                    Ok(())
                }
                None => Err(HueAPIError::BadResponse),
            },
            Err(_) => Err(HueAPIError::BadRequest),
        }
    }

    // pub(crate) async fn stream(&self) {
    //     use std::net::UdpSocket;
    //     let socket = UdpSocket::bind(self.entertainment_addr())?;
    // }

    #[cfg(feature = "sse")]
    pub(crate) async fn get_event_stream(&self) -> Result<EventSource, HueAPIError> {
        let req = self
            .client
            .request(Method::GET, self.event_stream_url())
            .header("hue-application-key", &self.app_key);

        match EventSource::new(req) {
            Ok(es) => Ok(es),
            Err(_) => Err(HueAPIError::ServerSentEvent),
        }
    }

    pub(crate) async fn get_bridge(&self) -> Result<BridgeData, HueAPIError> {
        let url = self.api_url() + "/resource/bridge";
        match self
            .make_request::<(), Vec<BridgeData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_bridge_home(
        &self,
        id: impl Into<String>,
    ) -> Result<HomeData, HueAPIError> {
        let url = self.api_url() + "/resource/bridge_home/" + &id.into();
        match self
            .make_request::<(), Vec<HomeData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_bridge_homes(&self) -> Result<Vec<HomeData>, HueAPIError> {
        let url = self.api_url() + "/resource/bridge_home";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_resources(&self) -> Result<Vec<Resource>, HueAPIError> {
        let url = self.api_url() + "/resource";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_behavior_script(
        &self,
        id: impl Into<String>,
    ) -> Result<BehaviorScriptData, HueAPIError> {
        let url = self.api_url() + "/resource/behavior_script/" + &id.into();
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_behavior_scripts(
        &self,
    ) -> Result<Vec<BehaviorScriptData>, HueAPIError> {
        let url = self.api_url() + "/resource/behavior_script";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_behavior_instance(
        &self,
        id: impl Into<String>,
    ) -> Result<BehaviorInstanceData, HueAPIError> {
        let url = self.api_url() + "/resource/behavior_instance/" + &id.into();
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_behavior_instances(
        &self,
    ) -> Result<Vec<BehaviorInstanceData>, HueAPIError> {
        let url = self.api_url() + "/resource/behavior_instance";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_behavior_instance(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/behavior_instance/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn post_behavior_instance(
        &self,
        payload: serde_json::Value,
    ) -> Result<ResourceIdentifier, HueAPIError> {
        let url = self.api_url() + "/resource/behavior_instance";
        let rids = self
            .make_request::<serde_json::Value, Vec<ResourceIdentifier>>(
                url,
                Method::POST,
                Some(payload.into()),
            )
            .await?;
        match rids.into_iter().nth(0) {
            Some(rid) => Ok(rid),
            None => Err(HueAPIError::BadDeserialize),
        }
    }

    pub(crate) async fn delete_behavior_instance(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/behavior_instance/" + &id.into();
        self.make_request(url, Method::DELETE, None::<()>).await
    }

    pub(crate) async fn get_entertainment_configuration(
        &self,
        id: impl Into<String>,
    ) -> Result<EntertainmentConfigurationData, HueAPIError> {
        let url = self.api_url() + "/resource/entertainment_configuration/" + &id.into();
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_entertainment_configurations(
        &self,
    ) -> Result<Vec<EntertainmentConfigurationData>, HueAPIError> {
        let url = self.api_url() + "/resource/entertainment_configuration";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_entertainment_configuration(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/entertainment_configuration/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn post_entertainment_configuration(
        &self,
        payload: serde_json::Value,
    ) -> Result<ResourceIdentifier, HueAPIError> {
        let url = self.api_url() + "/resource/entertainment_configuration";
        let rids = self
            .make_request::<serde_json::Value, Vec<ResourceIdentifier>>(
                url,
                Method::POST,
                Some(payload.into()),
            )
            .await?;
        match rids.into_iter().nth(0) {
            Some(rid) => Ok(rid),
            None => Err(HueAPIError::BadDeserialize),
        }
    }

    pub(crate) async fn delete_entertainment_configuration(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/entertainment_configuration/" + &id.into();
        self.make_request(url, Method::DELETE, None::<()>).await
    }

    pub(crate) async fn get_entertainment(
        &self,
        id: impl Into<String>,
    ) -> Result<EntertainmentData, HueAPIError> {
        let url = self.api_url() + "/resource/entertainment/" + &id.into();
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_entertainments(&self) -> Result<Vec<EntertainmentData>, HueAPIError> {
        let url = self.api_url() + "/resource/entertainment";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_button(
        &self,
        id: impl Into<String>,
    ) -> Result<ButtonData, HueAPIError> {
        let url = self.api_url() + "/resource/button/" + &id.into();
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_buttons(&self) -> Result<Vec<ButtonData>, HueAPIError> {
        let url = self.api_url() + "/resource/button";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_contact(
        &self,
        id: impl Into<String>,
    ) -> Result<ContactData, HueAPIError> {
        let url = self.api_url() + "/resource/contact/" + &id.into();
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_contacts(&self) -> Result<Vec<ContactData>, HueAPIError> {
        let url = self.api_url() + "/resource/contact";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_contact(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/contact/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_relative_rotary(
        &self,
        id: impl Into<String>,
    ) -> Result<RelativeRotaryData, HueAPIError> {
        let url = self.api_url() + "/resource/relative_rotary/" + &id.into();
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_relative_rotaries(
        &self,
    ) -> Result<Vec<RelativeRotaryData>, HueAPIError> {
        let url = self.api_url() + "/resource/relative_rotary";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_geolocation(
        &self,
        id: impl Into<String>,
    ) -> Result<GeolocationData, HueAPIError> {
        let url = self.api_url() + "/resource/geolocation/" + &id.into();
        match self
            .make_request::<(), Vec<GeolocationData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_geolocations(&self) -> Result<Vec<GeolocationData>, HueAPIError> {
        let url = self.api_url() + "/resource/geolocation";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_geolocation(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/geolocation/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_geofence_client(
        &self,
        id: impl Into<String>,
    ) -> Result<GeofenceClientData, HueAPIError> {
        let url = self.api_url() + "/resource/geofence_client/" + &id.into();
        match self
            .make_request::<(), Vec<GeofenceClientData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_geofence_clients(
        &self,
    ) -> Result<Vec<GeofenceClientData>, HueAPIError> {
        let url = self.api_url() + "/resource/geofence_client";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn post_geofence_client(
        &self,
        payload: serde_json::Value,
    ) -> Result<ResourceIdentifier, HueAPIError> {
        let url = self.api_url() + "/resource/geofence_client";
        let rids = self
            .make_request::<serde_json::Value, Vec<ResourceIdentifier>>(
                url,
                Method::POST,
                Some(payload.into()),
            )
            .await?;
        match rids.into_iter().nth(0) {
            Some(rid) => Ok(rid),
            None => Err(HueAPIError::BadDeserialize),
        }
    }

    pub(crate) async fn delete_geofence_client(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/geofence_client/" + &id.into();
        self.make_request(url, Method::DELETE, None::<()>).await
    }

    pub(crate) async fn put_geofence_client(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/geofence_client/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_tamper(
        &self,
        id: impl Into<String>,
    ) -> Result<TamperData, HueAPIError> {
        let url = self.api_url() + "/resource/tamper/" + &id.into();
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_tampers(&self) -> Result<Vec<TamperData>, HueAPIError> {
        let url = self.api_url() + "/resource/tamper";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_homekit(
        &self,
        id: impl Into<String>,
    ) -> Result<HomeKitData, HueAPIError> {
        let url = self.api_url() + "/resource/homekit/" + &id.into();
        match self
            .make_request::<(), Vec<HomeKitData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_homekits(&self) -> Result<Vec<HomeKitData>, HueAPIError> {
        let url = self.api_url() + "/resource/homekit";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_homekit(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/homekit/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_matter(
        &self,
        id: impl Into<String>,
    ) -> Result<MatterData, HueAPIError> {
        let url = self.api_url() + "/resource/matter/" + &id.into();
        match self
            .make_request::<(), Vec<MatterData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_matters(&self) -> Result<Vec<MatterData>, HueAPIError> {
        let url = self.api_url() + "/resource/matter";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_matter(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/matter/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_matter_fabric(
        &self,
        id: impl Into<String>,
    ) -> Result<MatterFabricData, HueAPIError> {
        let url = self.api_url() + "/resource/matter_fabric/" + &id.into();
        match self
            .make_request::<(), Vec<MatterFabricData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_matter_fabrics(&self) -> Result<Vec<MatterFabricData>, HueAPIError> {
        let url = self.api_url() + "/resource/matter_fabric";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn delete_matter_fabric(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/matter_fabric/" + &id.into();
        self.make_request(url, Method::DELETE, None::<()>).await
    }

    pub(crate) async fn get_motion(
        &self,
        id: impl Into<String>,
    ) -> Result<MotionData, HueAPIError> {
        let url = self.api_url() + "/resource/motion/" + &id.into();
        match self
            .make_request::<(), Vec<MotionData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_motions(&self) -> Result<Vec<MotionData>, HueAPIError> {
        let url = self.api_url() + "/resource/motion";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_motion(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/motion/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_camera_motion(
        &self,
        id: impl Into<String>,
    ) -> Result<MotionData, HueAPIError> {
        let url = self.api_url() + "/resource/camera_motion/" + &id.into();
        match self
            .make_request::<(), Vec<MotionData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_camera_motions(&self) -> Result<Vec<MotionData>, HueAPIError> {
        let url = self.api_url() + "/resource/camera_motion";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_camera_motion(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/camera_motion/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_device(
        &self,
        id: impl Into<String>,
    ) -> Result<DeviceData, HueAPIError> {
        let url = self.api_url() + "/resource/device/" + &id.into();
        match self
            .make_request::<(), Vec<DeviceData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_devices(&self) -> Result<Vec<DeviceData>, HueAPIError> {
        let url = self.api_url() + "/resource/device";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_device(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/device/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn delete_device(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/device/" + &id.into();
        self.make_request(url, Method::DELETE, None::<()>).await
    }

    pub(crate) async fn get_device_power(
        &self,
        id: impl Into<String>,
    ) -> Result<DevicePowerData, HueAPIError> {
        let url = self.api_url() + "/resource/device_power/" + &id.into();
        match self
            .make_request::<(), Vec<DevicePowerData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_device_powers(&self) -> Result<Vec<DevicePowerData>, HueAPIError> {
        let url = self.api_url() + "/resource/device_power";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_grouped_light(
        &self,
        id: impl Into<String>,
    ) -> Result<GroupData, HueAPIError> {
        let url = self.api_url() + "/resource/grouped_light/" + &id.into();
        match self
            .make_request::<(), Vec<GroupData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_grouped_lights(&self) -> Result<Vec<GroupData>, HueAPIError> {
        let url = self.api_url() + "/resource/grouped_light";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_grouped_light(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/grouped_light/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_light(&self, id: impl Into<String>) -> Result<LightData, HueAPIError> {
        let url = self.api_url() + "/resource/light/" + &id.into();
        match self
            .make_request::<(), Vec<LightData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_lights(&self) -> Result<Vec<LightData>, HueAPIError> {
        let url = self.api_url() + "/resource/light";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_light(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/light/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_room(&self, id: impl Into<String>) -> Result<ZoneData, HueAPIError> {
        let url = self.api_url() + "/resource/room/" + &id.into();
        match self
            .make_request::<(), Vec<ZoneData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_rooms(&self) -> Result<Vec<ZoneData>, HueAPIError> {
        let url = self.api_url() + "/resource/room";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_room(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/room/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn post_room(
        &self,
        payload: impl Into<serde_json::Value>,
    ) -> Result<ResourceIdentifier, HueAPIError> {
        let url = self.api_url() + "/resource/room";
        let rids = self
            .make_request::<serde_json::Value, Vec<ResourceIdentifier>>(
                url,
                Method::POST,
                Some(payload.into()),
            )
            .await?;
        match rids.into_iter().nth(0) {
            Some(rid) => Ok(rid),
            None => Err(HueAPIError::BadDeserialize),
        }
    }

    pub(crate) async fn delete_room(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/room/" + &id.into();
        self.make_request(url, Method::DELETE, None::<()>).await
    }

    pub(crate) async fn get_scene(&self, id: impl Into<String>) -> Result<SceneData, HueAPIError> {
        let url = self.api_url() + "/resource/scene/" + &id.into();
        match self
            .make_request::<(), Vec<SceneData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_scenes(&self) -> Result<Vec<SceneData>, HueAPIError> {
        let url = self.api_url() + "/resource/scene";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_scene(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/scene/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn post_scene(
        &self,
        payload: impl Into<serde_json::Value>,
    ) -> Result<ResourceIdentifier, HueAPIError> {
        let url = self.api_url() + "/resource/scene";
        let rids = self
            .make_request::<serde_json::Value, Vec<ResourceIdentifier>>(
                url,
                Method::POST,
                Some(payload.into()),
            )
            .await?;
        match rids.into_iter().nth(0) {
            Some(rid) => Ok(rid),
            None => Err(HueAPIError::BadDeserialize),
        }
    }

    pub(crate) async fn delete_scene(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/scene/" + &id.into();
        self.make_request(url, Method::DELETE, None::<()>).await
    }

    pub(crate) async fn get_smart_scene(
        &self,
        id: impl Into<String>,
    ) -> Result<SmartSceneData, HueAPIError> {
        let url = self.api_url() + "/resource/smart_scene/" + &id.into();
        match self
            .make_request::<(), Vec<SmartSceneData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_smart_scenes(&self) -> Result<Vec<SmartSceneData>, HueAPIError> {
        let url = self.api_url() + "/resource/smart_scene";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_smart_scene(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/smart_scene/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn post_smart_scene(
        &self,
        payload: impl Into<serde_json::Value>,
    ) -> Result<ResourceIdentifier, HueAPIError> {
        let url = self.api_url() + "/resource/smart_scene";
        let rids = self
            .make_request::<serde_json::Value, Vec<ResourceIdentifier>>(
                url,
                Method::POST,
                Some(payload.into()),
            )
            .await?;
        match rids.into_iter().nth(0) {
            Some(rid) => Ok(rid),
            None => Err(HueAPIError::BadDeserialize),
        }
    }

    pub(crate) async fn delete_smart_scene(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/smart_scene/" + &id.into();
        self.make_request(url, Method::DELETE, None::<()>).await
    }

    pub(crate) async fn get_light_level(
        &self,
        id: impl Into<String>,
    ) -> Result<LightLevelData, HueAPIError> {
        let url = self.api_url() + "/resource/light_level/" + &id.into();
        match self
            .make_request::<(), Vec<LightLevelData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_light_levels(&self) -> Result<Vec<LightLevelData>, HueAPIError> {
        let url = self.api_url() + "/resource/light_level";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_light_level(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/light_level/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_temperature(
        &self,
        id: impl Into<String>,
    ) -> Result<TemperatureData, HueAPIError> {
        let url = self.api_url() + "/resource/light_level/" + &id.into();
        match self
            .make_request::<(), Vec<TemperatureData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_temperatures(&self) -> Result<Vec<TemperatureData>, HueAPIError> {
        let url = self.api_url() + "/resource/temperature";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_temperature(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/temperature/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_zone(&self, id: impl Into<String>) -> Result<ZoneData, HueAPIError> {
        let url = self.api_url() + "/resource/zone/" + &id.into();
        match self
            .make_request::<(), Vec<ZoneData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_zones(&self) -> Result<Vec<ZoneData>, HueAPIError> {
        let url = self.api_url() + "/resource/zone";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_zone(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/zone/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn post_zone(
        &self,
        payload: impl Into<serde_json::Value>,
    ) -> Result<ResourceIdentifier, HueAPIError> {
        let url = self.api_url() + "/resource/zone";
        let rids = self
            .make_request::<serde_json::Value, Vec<ResourceIdentifier>>(
                url,
                Method::POST,
                Some(payload.into()),
            )
            .await?;
        match rids.into_iter().nth(0) {
            Some(rid) => Ok(rid),
            None => Err(HueAPIError::BadDeserialize),
        }
    }

    pub(crate) async fn delete_zone(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/zone/" + &id.into();
        self.make_request(url, Method::DELETE, None::<()>).await
    }

    pub(crate) async fn get_zgp_connectivity(
        &self,
        id: impl Into<String>,
    ) -> Result<ZGPConnectivityData, HueAPIError> {
        let url = self.api_url() + "/resource/zgp_connectivity" + &id.into();
        match self
            .make_request::<(), Vec<ZGPConnectivityData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_zgp_connectivities(
        &self,
    ) -> Result<Vec<ZGPConnectivityData>, HueAPIError> {
        let url = self.api_url() + "/resource/zgp_connectivity";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn get_zigbee_connectivity(
        &self,
        id: impl Into<String>,
    ) -> Result<ZigbeeConnectivityData, HueAPIError> {
        let url = self.api_url() + "/resource/zigbee_connectivity" + &id.into();
        match self
            .make_request::<(), Vec<ZigbeeConnectivityData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_zigbee_connectivities(
        &self,
    ) -> Result<Vec<ZigbeeConnectivityData>, HueAPIError> {
        let url = self.api_url() + "/resource/zigbee_connectivity";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_zigbee_connectivity(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/zigbee_connectivity/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }

    pub(crate) async fn get_zigbee_device_discovery(
        &self,
        id: impl Into<String>,
    ) -> Result<ZigbeeDeviceDiscoveryData, HueAPIError> {
        let url = self.api_url() + "/resource/zigbee_device_discovery" + &id.into();
        match self
            .make_request::<(), Vec<ZigbeeDeviceDiscoveryData>>(url, Method::GET, None::<()>)
            .await
        {
            Ok(data) => match data.into_iter().nth(0) {
                Some(first) => Ok(first),
                None => Err(HueAPIError::NotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn get_zigbee_device_discoveries(
        &self,
    ) -> Result<Vec<ZigbeeDeviceDiscoveryData>, HueAPIError> {
        let url = self.api_url() + "/resource/zigbee_device_discovery";
        self.make_request(url, Method::GET, None::<()>).await
    }

    pub(crate) async fn put_zigbee_device_discovery(
        &self,
        id: impl Into<String>,
        payload: &serde_json::Value,
    ) -> Result<Vec<ResourceIdentifier>, HueAPIError> {
        let url = self.api_url() + "/resource/zigbee_device_discovery/" + &id.into();
        self.make_request(url, Method::PUT, Some(payload)).await
    }
}

#[cfg(test)]
mod test {
    use crate::service::Resource;
    use include_dir::{include_dir, Dir};

    #[test]
    fn parse_resources() {
        static TEST_RESPONSES: Dir = include_dir!("test/fixtures/resource");

        for file in TEST_RESPONSES.files() {
            let data = file.contents_utf8().unwrap();
            let parsed: serde_json::Value = serde_json::from_str(&data).unwrap();
            let data = parsed["data"].as_array().unwrap();

            for resource in data {
                if let Err(e) = serde_json::from_value::<Resource>(resource.clone()) {
                    println!("{}", serde_json::to_string_pretty(resource).unwrap());
                    assert!(false, "Error: {}", e);
                }
            }
        }
    }
}
