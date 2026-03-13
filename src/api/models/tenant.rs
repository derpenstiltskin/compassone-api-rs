use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum TenantType {
    #[serde(rename(serialize = "MDR", deserialize = "Mdr"))]
    Mdr,
    #[serde(rename(serialize = "MDR ONBOARD", deserialize = "MdrOnboard"))]
    MdrOnboard,
    #[serde(rename(serialize = "POC", deserialize = "Poc"))]
    Poc,
    #[serde(rename(serialize = "SELF", deserialize = "TenantSelf"))]
    TenantSelf,
    #[serde(rename(serialize = "UNSET", deserialize = "Unset"))]
    Unset,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum TenantStatus {
    #[serde(rename(serialize = "Active", deserialize = "Active"))]
    Active,
    #[serde(rename(serialize = "Trial", deserialize = "Trial"))]
    Trial,
    #[serde(rename(serialize = "Unknown", deserialize = "Unknown"))]
    Unknown,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct TenantModel {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub account_id: Option<String>,
    pub r#type: TenantType,
    #[serde(default)]
    pub snap_agent_url: Option<String>,
    #[serde(default)]
    pub industry_type: Option<String>,
    pub enable_email_delivery: bool,
    #[serde(default)]
    pub contact_group_id: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    pub created: String,
    pub status: TenantStatus,
    pub informational_alerts_emails: Vec<String>,
    pub mdr_reports_emails: Vec<String>,
    pub dark_web_alerts_emails: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct TenantMinimalModel {
    pub id: String,
    pub name: String,
}
