use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum CloudMdrCiscoOnboardingStateType {
    #[serde(rename(serialize = "complete", deserialize = "Complete"))]
    Complete,
    #[serde(rename(serialize = "pending", deserialize = "pending"))]
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
pub struct CloudMdrCiscoOnboardingModel {
    id: String,
    created: String,
    connection_id: Option<String>,
    onboarding_id: Option<String>,
    error: Option<String>,
    state: CloudMdrCiscoOnboardingStateType,
    config: CloudMdrCiscoOnboardingConfigModel,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct CloudMdrCiscoOnboardingConfigModel {
    domain: String,
}
