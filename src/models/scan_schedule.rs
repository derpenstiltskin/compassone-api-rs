use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ScanType;

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ScanScheduleStatusType {
    #[serde(rename(serialize = "active", deserialize = "Active"))]
    Active,
    #[serde(rename(serialize = "completed", deserialize = "Completed"))]
    Completed,
    #[serde(rename(serialize = "disabled", deserialize = "Disabled"))]
    Disabled,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ScanScheduleFrequencyType {
    #[serde(rename(serialize = "daily", deserialize = "Daily"))]
    Daily,
    #[serde(rename(serialize = "monthly", deserialize = "Monthly"))]
    Monthly,
    #[serde(rename(serialize = "once", deserialize = "Once"))]
    Once,
    #[serde(rename(serialize = "weekly", deserialize = "Weekly"))]
    Weekly,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ScanScheduleModel {
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
    pub config: Option<Map<String, Value>>,
    #[serde(default)]
    pub asset_id: Option<String>,
    pub status: ScanScheduleStatusType,
    pub time: String,
    pub frequency: ScanScheduleFrequencyType,
    #[serde(default)]
    pub frequency_config: Option<Map<String, Value>>,
    pub name: String,
    #[serde(default)]
    pub job_key: Option<String>,
}
