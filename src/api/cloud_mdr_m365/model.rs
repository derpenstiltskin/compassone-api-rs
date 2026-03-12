use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct CloudMdrM365ConnectionModel {
    #[serde(default)]
    snap_name: Option<String>,
    #[serde(default)]
    snap_enabled: Option<String>,
    #[serde(default)]
    snap_package_id: Option<String>,
    #[serde(default)]
    updated: Option<String>,
    #[serde(default)]
    deleted: Option<String>,
    #[serde(default)]
    tenant_id: Option<String>,
    #[serde(default)]
    primary_domain: Option<String>,
    #[serde(default)]
    principal_id: Option<String>,
}
