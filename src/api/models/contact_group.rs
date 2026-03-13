use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ContactGroupModel {
    pub id: String,
    pub name: String,
    pub created: String,
    pub total_members: i64,
    pub total_assigned_tenants: i64,
    pub members: Vec<ContactGroupMemberModel>,
    pub assigned_tenants: Vec<ContactGroupMemberModel>,
}
