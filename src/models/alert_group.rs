use serde::{Deserialize, Serialize};

use super::AlertModel;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AlertGroupModel {
    pub id: String,
    pub customer_id: String,
    pub group_key: String,
    pub risk_score: i64,
    pub alert_count: i64,
    pub alert_types: Vec<String>,
    pub status: String,
    pub ticket_id: String,
    #[serde(default)]
    pub alert: Option<AlertModel>,
}
