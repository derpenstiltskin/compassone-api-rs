use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AlertModel {
    #[serde(default)]
    pub updated: Option<String>,
    pub id: String,
    pub customer_id: String,
    pub document_id: String,
    pub event_id: Option<String>,
    pub updated_by: Option<String>,
    pub risk_score: i64,
    pub alert_group_id: String,
    pub dataset: Option<String>,
    pub action: Option<String>,
    pub event_provider: Option<String>,
    pub username: Option<String>,
    pub hostname: Option<String>,
    pub device_id: Option<String>,
    pub rule_name: Option<String>,
    pub threat_framework: Option<String>,
    #[serde(default)]
    pub details: Option<Map<String, Value>>,
    #[serde(default)]
    pub anomaly_binary: Option<i64>,
    #[serde(default)]
    pub anomaly_percentile: Option<i64>,
    #[serde(default)]
    pub traffic_light: Option<String>,
    #[serde(default)]
    pub reasons: Option<Vec<String>>,
    #[serde(default)]
    pub soc_reporting_actions: Option<Vec<String>>,
    #[serde(default)]
    pub ai_ingestion_date: Option<String>,
    pub created: String,
}
