use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub struct DarkwebExposureModel {
    pub id: String,
    pub breach_id: String,
    pub breach_name: String,
    pub created_at: String,
    #[serde(default)]
    pub impacted_domain: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub password_type: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    pub scan_id: String,
}
