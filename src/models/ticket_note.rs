use serde::{Deserialize, Serialize};

use super::TicketStatusType;

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum TicketNoteType {
    #[serde(rename(serialize = "DETAIL_DESCRIPTION", deserialize = "DetailDescription"))]
    DetailDescription,
    #[serde(rename(serialize = "INTERNAL_ANALYSIS", deserialize = "InternalAnalysis"))]
    InternalAnalysis,
    #[serde(rename(serialize = "RESOLUTION", deserialize = "Resolution"))]
    Resolution,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct TicketNoteModel {
    #[serde(default)]
    pub updated: Option<String>,
    #[serde(default)]
    pub deleted: Option<String>,
    pub id: String,
    pub note: String,
    pub user: String,
    pub r#type: TicketNoteType,
    pub status: TicketStatusType,
    pub ticket_id: String,
    pub created: String,
}
