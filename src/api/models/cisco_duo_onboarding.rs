use serde::{Deserialize, Serialize};

use super::CiscoDuoOnboardingConfig;

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum CiscoDuoOnboardingStateType {
    #[serde(rename(serialize = "complete", deserialize = "Complete"))]
    Complete,
    #[serde(rename(serialize = "Pending", deserialize = "Pending"))]
    Pending,
    #[serde(rename(serialize = "usersOnboarded", deserialize = "UsersOnboarded"))]
    UsersOnboarded,
    #[serde(rename(serialize = "usersOnboardedError", deserialize = "UsersOnboardedError"))]
    UsersOnboardedError,
    #[serde(rename(serialize = "usersOnboarding", deserialize = "UsersOnboarding"))]
    UsersOnboarding,
    #[serde(rename(serialize = "verified", deserialize = "Verified"))]
    Verified,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct CiscoDuoOnboardingModel {
    pub id: String,
    pub created: String,
    pub connection_id: Option<String>,
    pub onboarding_id: Option<String>,
    pub error: Option<String>,
    pub state: CiscoDuoOnboardingStateType,
    pub config: CiscoDuoOnboardingConfigModel,
}
