use governor::{
    Quota, RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, direct::NotKeyed},
};
use reqwest::Client;
use std::{num::NonZeroU32, sync::Arc, time::Duration};

use crate::handlers::*;

const CO_API_REQUESTS_PER_HOUR: NonZeroU32 = NonZeroU32::new(2000).unwrap();
const CO_CLIENT_DEFAULT_USER_AGENT: &str = "compassone-api-rs";

pub enum CompassOneApiType {
    PRODUCTION,
    MOCK,
}

pub struct CompassOneApi {
    pub base_url: String,
    pub api_key: String,
    pub api_type: CompassOneApiType
}

pub struct CompassOneClient<'a> {
    client: Client,
    api: &'a CompassOneApi,
    user_agent: String,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl<'a> CompassOneClient<'a> {
    pub fn new(compass_one_client: &'a CompassOneApi, user_agent: Option<&str>) -> Self {
        Self {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            api: compass_one_client,
            user_agent: if let Some(user_agent) = user_agent {
                 user_agent.to_string()
            } else {
                CO_CLIENT_DEFAULT_USER_AGENT.to_string()
            },
            limiter: Arc::new(RateLimiter::direct(Quota::per_hour(CO_API_REQUESTS_PER_HOUR))),
        }
    }

    pub async fn request(&self) {

    }

    pub fn account(&self) -> AccountHandler<'_, '_> {
        AccountHandler::new(self)
    }

    pub fn asset(&self) -> AssetHandler<'_, '_> {
        AssetHandler::new(self)
    }

    pub fn cloud_mdr(&self) -> CloudMdr<'_, '_> {
        CloudMdr::new(self)
    }

    pub fn cloud_mdr_cisco(&self) -> CloudMdrCisco<'_, '_> {
        CloudMdrCisco::new(self)
    }

    pub fn cloud_mdr_google(&self) -> CloudMdrGoogle<'_, '_> {
        CloudMdrGoogle::new(self)
    }

    pub fn cloud_mdr_m365(&self) -> CloudMdrM365<'_, '_> {
        CloudMdrM365::new(self)
    }

    pub fn collection(&self) -> CollectionHandler<'_, '_> {
        CollectionHandler::new(self)
    }

    pub fn contact_group(&self) -> ContactGroupHandler<'_, '_> {
        ContactGroupHandler::new(self)
    }

    pub fn detections(&self) -> DetectionsHandler<'_, '_> {
        DetectionsHandler::new(self)
    }

    pub fn notifications(&self) -> NotificationsHandler<'_, '_> {
        NotificationsHandler::new(self)
    }

    pub fn tenant(&self) -> TenantHandler<'_, '_> {
        TenantHandler::new(self)
    }

    pub fn users(&self) -> UsersHandler<'_, '_> {
        UsersHandler::new(self)
    }

    pub fn vm_darkweb(&self) -> VmDarkwebHandler<'_, '_> {
        VmDarkwebHandler::new(self)
    }

    pub fn vm_external(&self) -> VmExternalHandler<'_, '_> {
        VmExternalHandler::new(self)
    }

    pub fn vm_scans(&self) -> VmScansHandler<'_, '_> {
        VmScansHandler::new(self)
    }

    pub fn vm_vulnerabilities(&self) -> VmVulnerabilitiesHandler<'_, '_> {
        VmVulnerabilitiesHandler::new(self)
    }
}
