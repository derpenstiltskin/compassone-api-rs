use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AssetRelationshipDirection {
    #[serde(rename(serialize = "in", deserialize = "In"))]
    In,
    #[serde(rename(serialize = "out", deserialize = "Out"))]
    Out,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AssetRelationshipDataStatus {
    #[serde(rename(serialize = "active", deserialize = "Active"))]
    Active,
    #[serde(rename(serialize = "closed", deserialize = "Closed"))]
    Closed,
    #[serde(rename(serialize = "inactive", deserialize = "Inactive"))]
    Inactive,
    #[serde(rename(serialize = "open", deserialize = "Open"))]
    Open,
    #[serde(rename(serialize = "suspended", deserialize = "Suspended"))]
    Suspended,
    #[serde(rename(serialize = "terminated", deserialize = "Terminated"))]
    Terminated,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AssetRelationshipDataType {
    #[serde(rename(serialize = "explicit", deserialize = "Explicit"))]
    Explicit,
    #[serde(rename(serialize = "implicit", deserialize = "Implicit"))]
    Implicit,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AssetRelationshipDataVerb {
    #[serde(rename(serialize = "connects", deserialize = "Connects"))]
    Connects,
    #[serde(rename(serialize = "has", deserialize = "Has"))]
    Has,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AssetRelationshipModel {
    data: AssetRelationshipDataModel,
    neighbor: Map<String, Value>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AssetRelationshipDataModel {
    pub account_id: String,
    #[serde(default)]
    pub created_by: Option<String>,
    pub created_on: String,
    pub tenant_id: String,
    #[serde(default)]
    pub deleted_by: Option<String>,
    #[serde(default)]
    pub deleted_on: Option<String>,
    #[serde(default)]
    pub found_by: Option<String>,
    pub found_on: String,
    pub from_id: String,
    pub id: String,
    #[serde(default)]
    pub last_seen_by: Option<String>,
    pub last_seen_on: String,
    pub status: AssetRelationshipDataStatus,
    pub to_id: String,
    #[serde(default)]
    pub r#type: Option<AssetRelationshipDataType>,
    #[serde(default)]
    pub updated_by: Option<String>,
    #[serde(default)]
    pub updated_on: Option<String>,
    pub verb: AssetRelationshipDataVerb,
}
