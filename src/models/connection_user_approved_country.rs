use serde::{Deserialize, Serialize};

use super::IsoCountryModel;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ConnectionApprovedCountryModel {
    pub id: String,
    pub connection_id: String,
    pub iso_country: IsoCountryModel,
    pub created: String,
    pub end_state: Option<String>,
    pub start_date: Option<String>,
}
