use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ScanAndScheduleStatusType {
    #[serde(rename(serialize = "active", deserialize = "Active"))]
    Active,
    #[serde(rename(serialize = "completed", deserialize = "Completed"))]
    Completed,
    #[serde(rename(serialize = "disabled", deserialize = "Disabled"))]
    Disabled,
    #[serde(rename(serialize = "canceled", deserialize = "Canceled"))]
    Canceled,
    #[serde(rename(serialize = "failed", deserialize = "Failed"))]
    Failed,
    #[serde(rename(serialize = "in-progress", deserialize = "InProgress"))]
    InProgress,
    #[serde(rename(serialize = "new", deserialize = "New"))]
    New,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ScanAndScheduleSourceTableType {
    #[serde(rename(serialize = "scan", deserialize = "Scan"))]
    Scan,
    #[serde(rename(serialize = "scanschedule", deserialize = "ScanSchedule"))]
    ScanSchedule,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ScanAndScheduleFrequencyType {
    #[serde(rename(serialize = "daily", deserialize = "Daily"))]
    Daily,
    #[serde(rename(serialize = "monthly", deserialize = "Monthly"))]
    Monthly,
    #[serde(rename(serialize = "once", deserialize = "Once"))]
    Once,
    #[serde(rename(serialize = "weekly", deserialize = "Weekly"))]
    Weekly,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ScanAndScheduleType {
    #[serde(rename(serialize = "darkweb", deserialize = "darkweb"))]
    Darkweb,
    #[serde(rename(serialize = "external", deserialize = "external"))]
    External,
    #[serde(rename(serialize = "local", deserialize = "local"))]
    Local,
    #[serde(rename(serialize = "network", deserialize = "Network"))]
    Network,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ScanAndScheduleModel {
    pub status: ScanAndScheduleStatusType,
    pub source_table: ScanAndScheduleSourceTableType,
    pub id: String,
    pub account_id: String,
    #[serde(default)]
    pub created_by: Option<String>,
    pub created_on: String,
    pub tenant_id: String,
    #[serde(default)]
    pub updated_on: Option<String>,
    pub time: String,
    pub frequency: ScanAndScheduleFrequencyType,
    pub frequency_config: Option<Map<String, Value>>,
    pub name: String,
    pub r#type: ScanAndScheduleType,
    #[serde(default)]
    pub scans_count: i64,
}
