use governor::{
    Quota, RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, direct::NotKeyed},
};
use reqwest::Client;
use std::{num::NonZeroU32, sync::Arc, time::Duration};

use crate::api::handlers::*;

const CO_API_PROD_BASE_URL: &str = "";
const CO_API_MOCK_BASE_URL: &str = "";
const CO_API_REQUESTS_PER_HOUR: NonZeroU32 = NonZeroU32::new(2000).unwrap();
const CO_CLIENT_USER_AGENT: &str = "compassone-api-rs";

pub enum CompassOneClientType {
    PRODUCTION,
    MOCK,
}

pub struct CompassOneClient {
    client: Client,
    base_url: String,
    api_key: String,
    user_agent: String,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl CompassOneClient {
    pub fn new(client_type: CompassOneClientType, api_key: &str, user_agent: Option<&str>) -> Self {
        Self {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            base_url: match client_type {
                CompassOneClientType::PRODUCTION => CO_API_PROD_BASE_URL.to_string(),
                CompassOneClientType::MOCK => CO_API_MOCK_BASE_URL.to_string(),
            },
            api_key: api_key.to_string(),
            user_agent: if let Some(user_agent) = user_agent {
                user_agent.to_string()
            } else {
                CO_CLIENT_USER_AGENT.to_string()
            },
            limiter: Arc::new(RateLimiter::direct(Quota::per_hour(
                CO_API_REQUESTS_PER_HOUR,
            ))),
        }
    }

    pub fn account(&self) -> AccountHandler<'_> {
        AccountHandler::new(self)
    }

    pub fn asset(&self) -> AssetHandler<'_> {
        AssetHandler::new(self)
    }

    pub fn cloud_mdr(&self) -> CloudMdr<'_> {
        CloudMdr::new(self)
    }

    pub fn cloud_mdr_cisco(&self) -> CloudMdrCisco<'_> {
        CloudMdrCisco::new(self)
    }

    pub fn cloud_mdr_google(&self) -> CloudMdrGoogle<'_> {
        CloudMdrGoogle::new(self)
    }

    pub fn cloud_mdr_m365(&self) -> CloudMdrM365<'_> {
        CloudMdrM365::new(self)
    }

    pub fn collection(&self) -> CollectionHandler<'_> {
        CollectionHandler::new(self)
    }

    pub fn contact_group(&self) -> ContactGroupHandler<'_> {
        ContactGroupHandler::new(self)
    }

    pub fn detections(&self) -> DetectionsHandler<'_> {
        DetectionsHandler::new(self)
    }

    pub fn notifications(&self) -> NotificationsHandler<'_> {
        NotificationsHandler::new(self)
    }

    pub fn tenant(&self) -> TenantHandler<'_> {
        TenantHandler::new(self)
    }

    pub fn users(&self) -> UsersHandler<'_> {
        UsersHandler::new(self)
    }

    pub fn vm_darkweb(&self) -> VmDarkwebHandler<'_> {
        VmDarkwebHandler::new(self)
    }

    pub fn vm_external(&self) -> VmExternalHandler<'_> {
        VmExternalHandler::new(self)
    }

    pub fn vm_scans(&self) -> VmScansHandler<'_> {
        VmScansHandler::new(self)
    }

    pub fn vm_vulnerabilities(&self) -> VmVulnerabilities<'_> {
        VmVulnerabilities::new(self)
    }
}
