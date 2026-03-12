use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
#[non_exhaustive]
pub enum CloudMdrGoogleOnboardingStateType {
    #[serde(rename(serialize = "complete", deserialize = "Complete"))]
    Complete,
    #[serde(rename(serialize = "pendingConsent", deserialize = "PendingConsent"))]
    PendingConsent,
    #[serde(rename(serialize = "pendingDomainWideDelegationVerification"))]
    PendingDomainWideDelegationVerification,
    #[serde(rename(serialize = "provisionedApis", deserialize = "ProvisionedApis"))]
    ProvisionedApis,
    #[serde(rename(serialize = "provisionedProject", deserialize = "ProvisionedProject"))]
    ProvisionedProject,
    #[serde(rename(serialize = "provisionedRefreshToken", deserialize = "ProvisionedRefreshToken"))]
    ProvisionedRefreshToken,
    #[serde(rename(serialize = "provisionedRefreshTokenError", deserialize = "ProvisionedRefreshTokenError"))]
    ProvisionedRefreshTokenError,
    #[serde(rename(serialize = "provisionedServiceAccount", deserialize = "ProvisionedServiceAccount"))]
    ProvisionedServiceAccount,
    #[serde(rename(serialize = "provisionedServiceAccountKey", deserialize = "ProvisionedServiceKeyAccount"))]
    ProvisionedServiceAccountKey,
    #[serde(rename(serialize = "provisionedServiceAccountKeyError", deserialize = "ProvisionedServiceAccountKeyError"))]
    ProvisionedServiceAccountKeyError,
    #[serde(rename(serialize = "provisionApisError", deserialize = "ProvisionApisError"))]
    ProvisionApisError,
    #[serde(rename(serialize = "provisionProjectError", deserialize = "ProvisionProjectError"))]
    ProvisionProjectError,
    #[serde(rename(serialize = "provisionServiceAccountError", deserialize = "ProvisionServiceAccountError"))]
    ProvisionServiceAccountError,
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
pub struct CloudMdrGoogleOnboardingModel {
    consent_url: Option<String>,
    domain_wide_delegation: Option<String>,
    config: CloudMdrGoogleOnboardingConfigModel,
    id: String,
    created: String,
    connection_id: Option<String>,
    error: Option<String>,
    state: CloudMdrGoogleOnboardingStateType,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct CloudMdrGoogleOnboardingConfigModel {
    redirect_path: String,
    domain: String,
}
