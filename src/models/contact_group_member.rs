use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum ContactGroupMemberAvailabilityType {
    #[serde(rename(serialize = "After Hours", deserialize = "AfterHours"))]
    AfterHours,
    #[serde(rename(serialize = "All Hours", deserialize = "AllHours"))]
    AllHours,
    #[serde(rename(serialize = "Business Hours", deserialize = "BusinessHours"))]
    BusinessHours,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ContactGroupMemberModel {
    pub id: String,
    pub name: String,
    pub phone_number: String,
    pub email: String,
    pub availability: ContactGroupMemberAvailabilityType,
    pub timezone: String,
    pub priority: i64,
}
