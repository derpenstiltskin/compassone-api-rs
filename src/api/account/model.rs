use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AccountPartnershipType {
    Direct,
    MSP,
    MSSP,
    VAR,
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
    id: String,
    name: String,
    vendor_id: String,
    partnership_type: Option<AccountPartnershipType>,
    billing_version: Option<AccountBillingVersion>,
    #[serde(default)]
    onboarded_date: Option<String>,
    branding: Option<AccountBrandingModel>,
    created: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AccountBrandingModel {
    #[serde(default)]
    company_website: Option<String>,
    #[serde(default)]
    sales_email: Option<String>,
    #[serde(default)]
    sales_phone_number: Option<String>,
    #[serde(default)]
    company_logo: Option<AccountBrandingCompanyLogoModel>
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AccountBrandingCompanyLogoModel {
    base64: String,
    content_type: String,
}
