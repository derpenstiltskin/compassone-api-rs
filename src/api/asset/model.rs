use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AssetModel {
    id: String,
    asset_class: String,
    #[serde(default)]
    classification: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    display_name: Option<String>,
    name: String,
    #[serde(default)]
    notes: Option<String>,
    #[serde(default)]
    production: Option<bool>,
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    account_id: Option<String>,
    #[serde(default)]
    created_by: Option<String>,
    created_on: String,
    #[serde(default)]
    criticality: Option<i8>,
    tenant_id: String,
    #[serde(default)]
    deleted_by: Option<String>,
    #[serde(default)]
    deleted_on: Option<String>,
    #[serde(default)]
    found_by: Option<String>,
    found_on: String,
    #[serde(default)]
    last_seen_by: Option<String>,
    last_seen_on: String,
    #[serde(default)]
    updated_by: Option<String>,
    model: String,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    updated_on: Option<String>,
    tags: Vec<AssetTagModel>,
    sources: Vec<AssetSourceModel>,
    #[serde(default)]
    command: Option<String>,
    #[serde(default)]
    container_id: Option<String>,
    #[serde(default)]
    ports: Option<Vec<String>>,
    #[serde(default)]
    image_tag: Option<String>,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    started_on: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AssetTagModel {
    id: String,
    name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AssetSourceModel {
    id: String,
    #[serde(default)]
    display_name: Option<String>,
    name: String,
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    external_id: Option<String>,
}
