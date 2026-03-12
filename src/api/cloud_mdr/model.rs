use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct CloudMdrCountryModel {
    code: String,
    name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct CloudMdrConnectionUser {
    name: Option<String>,
    email: Option<String>,
    id: String,
    active_approved_countries: Vec<CloudMdrConnectionUserApprovedCountry>,
    upcoming_approved_countries: Vec<CloudMdrConnectionUserApprovedCountry>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct CloudMdrConnectionUserApprovedCountry {
    id: String,
    connection_id: String,
    iso_country: CloudMdrCountryModel,
    created: String,
    end_state: Option<String>,
    start_date: Option<String>,
}
