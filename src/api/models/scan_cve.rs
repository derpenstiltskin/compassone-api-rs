use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ScanCveExpoitabilityType {
    #[serde(rename(serialize = "Attacked", deserialize = "Attacked"))]
    Attacked,
    #[serde(rename(serialize = "Unreported", deserialize = "Unreported"))]
    Unreported,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ScanCveModel {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub severity: String,
    pub base_score: i64,
    pub asset_environmental_score: i64,
    pub prioritized: bool,
    pub host: String,
    pub app: String,
    pub scan_cve_id: String,
    pub url: String,
    #[serde(default)]
    pub found_on: Option<String>,
    #[serde(default)]
    pub application_name: Option<String>,
    #[serde(default)]
    pub application_family: Option<String>,
    #[serde(default)]
    pub application_vendor: Option<String>,
    #[serde(default)]
    pub remediation: Option<String>,
    #[serde(default)]
    pub remediation_url: Option<String>,
    pub expoitability: ScanCveExploitabilityType,
    #[serde(default)]
    pub vector_string: Option<String>,
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub tenant_id: Option<String>,
}
