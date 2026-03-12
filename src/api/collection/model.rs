use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum CollectionContextType {
    #[serde(rename(serialize = "ASSET", deserialize = "Asset"))]
    Asset,
    #[serde(rename(serialize = "FINDING", deserialize = "Finding"))]
    Finding,
    #[serde(rename(serialize = "CONTAINER", deserialize = "Container"))]
    Container,
    #[serde(rename(serialize = "DEVICE", deserialize = "Device"))]
    Device,
    #[serde(rename(serialize = "FRAMEWORK", deserialize = "Framework"))]
    Framework,
    #[serde(rename(serialize = "NETSTAT", deserialize = "NetStat"))]
    NetStat,
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
    #[serde(rename(serialize = "USER", deserialize = "User"))]
    User,
    #[serde(rename(serialize = "ALERT", deserialize = "Alert"))]
    Alert,
    #[serde(rename(serialize = "ALERTGROUP", deserialize = "AlertGroup"))]
    AlertGroup,
    #[serde(rename(serialize = "EVENT", deserialize = "Event"))]
    Event,
    #[serde(rename(serialize = "INCIDENT", deserialize = "Incident"))]
    Incident,
    #[serde(rename(serialize = "VULNERABILITY", deserialize = "Vulnerabiliuty"))]
    Vulnerability,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct CollectionModel {
    id: String,
    context: CollectionContextType,
    #[serde(default)]
    created_by: Option<String>,
    created_on: String,
    name: String,
    search: String,
    #[serde(default)]
    updated_by: Option<String>,
    #[serde(default)]
    updated_on: Option<String>,
    user_id: String,
}
