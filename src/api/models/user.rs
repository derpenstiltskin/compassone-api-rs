use serde::{Deserialize, Serialize};

use super::{AccountMinimalModel, TenantMinimalModel};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum UserRoleType {
    #[serde(rename(serialize = "AccountAdmin", deserialize = "AccountAdmin"))]
    AccountAdmin,
    #[serde(rename(
        serialize = "AccountBillingAbility",
        deserialize = "AccountBillingAbility"
    ))]
    AccountBillingAbility,
    #[serde(rename(serialize = "AccountUser", deserialize = "AccountUser"))]
    AccountUser,
    #[serde(rename(serialize = "BlackpointAdmin", deserialize = "BlackpointAdmin"))]
    BlackpointAdmin,
    #[serde(rename(
        serialize = "BlackpointSuperAdmin",
        deserialize = "BlackpointSuperAdmin"
    ))]
    BlackpointSuperAdmin,
    #[serde(rename(serialize = "BlackpointUser", deserialize = "BlackpointUser"))]
    BlackpointUser,
    #[serde(rename(serialize = "CustomerAdmin", deserialize = "CustomerAdmin"))]
    CustomerAdmin,
    #[serde(rename(serialize = "CustomerUser", deserialize = "CustomerUser"))]
    CustomerUser,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct UserModel {
    pub id: String,
    pub email: String,
    pub name: String,
    pub billing_access: bool,
    pub roles: UserRoleType,
    #[serde(default)]
    pub assigned_accounts: Option<Vec<AccountMinimalModel>>,
    pub assigned_tenants: Option<Vec<TenantMinimalModel>>,
}
