use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ScanStatisticsModel {
    pub completed: i64,
    pub darkweb: i64,
    pub external: i64,
    pub failed: i64,
    pub local: i64,
    pub network: i64,
    pub scheduled: i64,
    pub scheduled_active: i64,
    pub total: i64,
}
