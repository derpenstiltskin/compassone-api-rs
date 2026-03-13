use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct CustomerModel {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub ticket_system_id: Option<String>,
    #[serde(default)]
    pub ticket_system_primary_contact_id: Option<String>,
    #[serde(default)]
    pub ticket_system_contact_group_id: Option<String>,
    #[serde(default)]
    pub onboard_ticket_id: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub is_eu_customer: Option<bool>,
    #[serde(default)]
    pub exclude_from_ai: Option<bool>,
    #[serde(default)]
    pub contact_group_id: Option<String>,
    #[serde(default)]
    pub robocall_eligible: Option<bool>,
    pub created: String,
    #[serde(default)]
    pub updated: Option<String>,
    #[serde(default)]
    pub deleted: Option<String>,
    pub account_id: Option<String>,
}
