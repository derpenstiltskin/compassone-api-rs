use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ScanType {
    #[serde(rename(serialize = "darkweb", deserialize = "Darkweb"))]
    Darkweb,
    #[serde(rename(serialize = "external", deserialize = "External"))]
    External,
    #[serde(rename(serialize = "local", deserialize = "Local"))]
    Local,
    #[serde(rename(serialize = "network", deserialize = "Network"))]
    Network,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ScanTriggeredByType {
    #[serde(rename(serialize = "schedule", deserialize = "Schedule"))]
    Schedule,
    #[serde(rename(serialize = "system", deserialize = "System"))]
    System,
    #[serde(rename(serialize = "user", deserialize = "User"))]
    User,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ScanStatusType {
    #[serde(rename(serialize = "canceled", deserialize = "Canceled"))]
    Canceled,
    #[serde(rename(serialize = "completed", deserialize = "Completed"))]
    Completed,
    #[serde(rename(serialize = "failed", deserialize = "Failed"))]
    Failed,
    #[serde(rename(serialize = "in-progress", deserialize = "InProgress"))]
    InProgress,
    #[serde(rename(serialize = "new", deserialize = "New"))]
    New,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub struct ScanModel {
    pub id: String,
    pub account_id: String,
    #[serde(default)]
    pub created_by: Option<String>,
    pub created_on: String,
    pub tenant_id: String,
    #[serde(default)]
    pub deleted_by: Option<String>,
    #[serde(default)]
    pub deleted_on: Option<String>,
    #[serde(default)]
    pub updated_by: Option<String>,
    #[serde(default)]
    pub updated_on: Option<String>,
    pub r#type: ScanType,
    #[serde(default)]
    pub config: Map<String, Value>,
    #[serde(default)]
    pub result: Map<String, Value>,
    #[serde(default)]
    pub asset_id: Option<String>,
    pub triggered_by_type: ScanTriggeredByType,
    #[serde(default)]
    pub triggered_by: Option<String>,
    pub status: ScanStatusType,
    #[serde(default)]
    pub triggered_by_name: Option<String>,
}
