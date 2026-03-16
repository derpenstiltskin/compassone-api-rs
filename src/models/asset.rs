use serde::{Deserialize, Serialize};

use super::{AssetSourceModel, AssetTagModel};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AssetClass {
    #[serde(rename(serialize = "CONTAINER", deserialize = "Container"))]
    Container,
    #[serde(rename(serialize = "DEVICE", deserialize = "Device"))]
    Device,
    #[serde(rename(serialize = "FRAMEWORK", deserialize = "Framework"))]
    Framework,
    #[serde(rename(serialize = "NETSTAT", deserialize = "Netstat"))]
    Netstat,
    #[serde(rename(serialize = "PERSON", deserialize = "Person"))]
    Person,
    #[serde(rename(serialize = "PROCESS", deserialize = "Process"))]
    Process,
    #[serde(rename(serialize = "SERVICE", deserialize = "Service"))]
    Service,
    #[serde(rename(serialize = "SOFTWARE", deserialize = "Software"))]
    Software,
    #[serde(rename(serialize = "SOURCE", deserialize = "Source"))]
    Source,
    #[serde(rename(serialize = "SURVEY", deserialize = "Survey"))]
    Survey,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AssetModel {
    pub id: String,
    pub asset_class: String,
    #[serde(default)]
    pub classification: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    pub name: String,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub production: Option<bool>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub created_by: Option<String>,
    pub created_on: String,
    #[serde(default)]
    pub criticality: Option<i8>,
    pub tenant_id: String,
    #[serde(default)]
    pub deleted_by: Option<String>,
    #[serde(default)]
    pub deleted_on: Option<String>,
    #[serde(default)]
    pub found_by: Option<String>,
    pub found_on: String,
    #[serde(default)]
    pub last_seen_by: Option<String>,
    pub last_seen_on: String,
    #[serde(default)]
    pub updated_by: Option<String>,
    pub model: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub updated_on: Option<String>,
    pub tags: Vec<AssetTagModel>,
    pub sources: Vec<AssetSourceModel>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub container_id: Option<String>,
    #[serde(default)]
    pub ports: Option<Vec<String>>,
    #[serde(default)]
    pub image_tag: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub started_on: Option<String>,
}
