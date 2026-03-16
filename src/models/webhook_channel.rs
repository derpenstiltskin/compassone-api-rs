use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct WebhookChannelModel {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub url: String,
    pub api_secret_name_header: String,
    pub headers: Map<String, Value>,
    pub account_id: String,
    pub tenant_id: Option<String>,
    pub created: String,
    pub updated: Option<String>,
}
