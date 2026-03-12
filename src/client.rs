use governor::{
    Quota,
    RateLimiter,
    state::{
        direct::NotKeyed,
        InMemoryState,
    },
    clock::DefaultClock,
};
use reqwest::Client;
use std::{
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};

use crate::api::{
    account,
    asset,
    cloud_mdr,
    cloud_mdr_cisco,
    cloud_mdr_google,
    cloud_mdr_m365,
    collection,
    contact_group,
    detections,
    notifications,
    tenant,
    users,
    vm_darkweb,
    vm_external,
    vm_scans,
    vm_vulnerabilities,
};

const CO_API_PROD_BASE_URL: &str = "https://api.blackpointcyber.com";
const CO_API_MOCK_BASE_URL: &str = "https://docs.blackpointcyber.com/_mock/apis/compassone-api/";
const CO_API_REQUESTS_PER_HOUR: NonZeroU32 = NonZeroU32::new(2000).unwrap();
const CO_CLIENT_USER_AGENT: &str = "compassone-api-rs";

pub enum CompassOneClientType {
    PRODUCTION,
    MOCK
}

pub struct CompassOneClient {
    client: Client,
    base_url: String,
    api_key: String,
    user_agent: String,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl CompassOneClient {
    pub fn new(
        client_type: CompassOneClientType,
        api_key: &str,
        user_agent: Option<&str>
    ) -> Self {
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
            limiter: Arc::new(RateLimiter::direct(Quota::per_hour(CO_API_REQUESTS_PER_HOUR))),
        }
    }

    pub fn account(&self) -> account::handler::AccountHandler<'_> {
        account::handler::AccountHandler::new(self)
    }

    pub fn asset(&self) -> asset::handler::AssetHandler<'_> {
        asset::handler::AssetHandler::new(self)
    }

    pub fn cloud_mdr(&self) -> cloud_mdr::handler::CloudMdr<'_> {
        cloud_mdr::handler::CloudMdr::new(self)
    }

    pub fn cloud_mdr_cisco(&self) -> cloud_mdr_cisco::handler::CloudMdrCisco<'_> {
        cloud_mdr_cisco::handler::CloudMdrCisco::new(self)
    }

    pub fn cloud_mdr_google(&self) -> cloud_mdr_google::handler::CloudMdrGoogle<'_> {
        cloud_mdr_google::handler::CloudMdrGoogle::new(self)
    }

    pub fn cloud_mdr_m365(&self) -> cloud_mdr_m365::handler::CloudMdrM365<'_> {
        cloud_mdr_m365::handler::CloudMdrM365::new(self)
    }

    pub fn collection(&self) -> collection::handler::CollectionHandler<'_> {
        collection::handler::CollectionHandler::new(self)
    }

    pub fn contact_group(&self) -> contact_group::handler::ContactGroupHandler<'_> {
        contact_group::handler::ContactGroupHandler::new(self)
    }

    pub fn detections(&self) -> detections::handler::DetectionsHandler<'_> {
        detections::handler::DetectionsHandler::new(self)
    }

    pub fn notifications(&self) -> notifications::handler::NotificationsHandler<'_> {
        notifications::handler::NotificationsHandler::new(self)
    }

    pub fn tenant(&self) -> tenant::handler::TenantHandler<'_> {
        tenant::handler::TenantHandler::new(self)
    }

    pub fn users(&self) -> users::handler::UsersHandler<'_> {
        users::handler::UsersHandler::new(self)
    }

    pub fn vm_darkweb(&self) -> vm_darkweb::handler::VmDarkwebHandler<'_> {
        vm_darkweb::handler::VmDarkwebHandler::new(self)
    }

    pub fn vm_external(&self) -> vm_external::handler::VmExternalHandler<'_> {
        vm_external::handler::VmExternalHandler::new(self)
    }

    pub fn vm_scans(&self) -> vm_scans::handler::VmScansHandler<'_> {
        vm_scans::handler::VmScansHandler::new(self)
    }

    pub fn vm_vulnerabilities(&self) -> vm_vulnerabilities::handler::VmVulnerabilities<'_> {
        vm_vulnerabilities::handler::VmVulnerabilities::new(self)
    }
}
