use serde::{Deserialize, Serialize};

use super::ConnectionUserApprovedCountryModel;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ConnectionUserModel {
    pub name: Option<String>,
    pub email: Option<String>,
    pub id: String,
    pub active_approved_countries: Vec<ConnectionUserApprovedCountryModel>,
    pub upcoming_approved_countries: Vec<ConnectionUserApprovedCountryModel>,
}
