use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AccountPartnershipType {
    #[serde(rename(serialize = "Direct", deserialize = "Direct"))]
    Direct,
    #[serde(rename(serialize = "MSP", deserialize = "Msp"))]
    Msp,
    #[serde(rename(serialize = "MSSP", deserialize = "Mssp"))]
    Mssp,
    #[serde(rename(serialize = "VAR", deserialize = "Var"))]
    Var,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AccountBillingVersion {
    #[serde(rename(serialize = "1", deserialize = "One"))]
    One,
    #[serde(rename(serialize = "2", deserialize = "Two"))]
    Two,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AccountModel {
    pub id: String,
    pub name: String,
    pub vendor_id: String,
    pub partnership_type: Option<AccountPartnershipType>,
    pub billing_version: Option<AccountBillingVersion>,
    #[serde(default)]
    pub onboarded_date: Option<String>,
    pub branding: Option<AccountBrandingModel>,
    pub created: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AccountBrandingModel {
    #[serde(default)]
    pub company_website: Option<String>,
    #[serde(default)]
    pub sales_email: Option<String>,
    #[serde(default)]
    pub sales_phone_number: Option<String>,
    #[serde(default)]
    pub company_logo: Option<AccountBrandingCompanyLogoModel>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AccountBrandingCompanyLogoModel {
    pub base64: String,
    pub content_type: String,
}
