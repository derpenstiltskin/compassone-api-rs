use serde::{Deserialize, Serialize};

use super::TicketNotesModel;

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum TicketStatusType {
    #[serde(rename(serialize = "CLAIM", deserialize = "Claim"))]
    Claim,
    #[serde(rename(serialize = "CLOSE", deserialize = "Close"))]
    Close,
    #[serde(rename(serialize = "ESCALATE", deserialize = "Escalate"))]
    Escalate,
    #[serde(rename(serialize = "INFORMATIONAL", deserialize = "Informational"))]
    Informational,
    #[serde(rename(serialize = "INVESTIGATE", deserialize = "Investigate"))]
    Investigate,
    #[serde(rename(serialize = "NEW", deserialize = "New"))]
    New,
    #[serde(rename(serialize = "NO_AGENT", deserialize = "NoAgent"))]
    NoAgent,
    #[serde(rename(serialize = "RESOLVE", deserialize = "Resolve"))]
    Resolve,
    #[serde(rename(serialize = "SUGGEST_SUPPRESSION", deserialize = "SuggestSuppression"))]
    SuggestSuppression,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct TicketModel {
    #[serde(default)]
    pub updated: Option<String>,
    #[serde(default)]
    pub deleted: Option<String>,
    pub id: String,
    pub ticket_system_id: Option<String>,
    pub status: TicketStatusType,
    pub customer_id: String,
    pub external: bool,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    pub notes: Option<Vec<TicketNotesModel>>,
    pub created: String,
}
