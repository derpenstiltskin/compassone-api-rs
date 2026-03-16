use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct EmailChannelModel {
    pub id: String,
    pub emails: Vec<String>,
    pub name: String,
    pub enabled: bool,
    pub account_id: String,
    pub tenant_id: Option<String>,
    pub created: String,
    pub updated: Option<String>,
}
