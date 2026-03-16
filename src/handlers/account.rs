use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    models::{
        AccountBillingVersion,
        AccountPartnershipType,
    },
    handlers::GenericListSortOrder,
};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AccountListSortBy {
    #[serde(rename(serialize = "id", deserialize = "Id"))]
    Id,
    #[serde(rename(serialize = "name", deserialize = "Name"))]
    Name,
    #[serde(rename(serialize = "createdBy", deserialize = "CreatedBy"))]
    CreatedBy,
    #[serde(rename(serialize = "partnershipType", deserialize = "PartnershipType"))]
    PartnershipType,
    #[serde(rename(serialize = "billingOrder", deserialize = "BillingOrder"))]
    BillingVersion,
}

pub struct AccountHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> AccountHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_account(&self, id: &str) -> AccountGetBuilder<'_, '_, '_> {
        AccountGetBuilder::new(self, id)
    }

    pub fn list_accounts(&self) -> AccountListBuilder<'_, '_, '_> {
        AccountListBuilder::new(self)
    }
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AccountGetBuilder<'c, 'h, 'r> {
    #[serde(skip)]
    handler: &'r AccountHandler<'c, 'h>,
    #[serde(skip)]
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_branding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_logo: Option<bool>,
}

impl<'c, 'h, 'r> AccountGetBuilder<'c, 'h, 'r> {
    pub(crate) fn new(handler: &'r AccountHandler<'c, 'h>, id: &str) -> Self {
        Self {
            handler,
            id: id.to_string(),
            include_branding: None,
            include_logo: None,
        }
    }

    pub fn include_branding(mut self, include_branding: bool) -> Self {
        self.include_branding = Some(include_branding);
        self
    }

    pub fn include_logo(mut self, include_logo: bool) -> Self {
        self.include_logo = Some(include_logo);
        self
    }

    pub async fn send(self) {
        let route = format!("/accounts/{}", self.id);
    }
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AccountListBuilder<'c, 'h, 'r> {
    #[serde(skip)]
    handler: &'r AccountHandler<'c, 'h>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<AccountListSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<GenericListSortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_version: Option<AccountBillingVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partnership_type: Option<AccountPartnershipType>,
}

impl<'c, 'h, 'r> AccountListBuilder<'c, 'h, 'r> {
    pub(crate) fn new(handler: &'r AccountHandler<'c, 'h>) -> Self {
        Self {
            handler,
            page_size: None,
            page: None,
            search: None,
            sort_by: None,
            sort_order: None,
            billing_version: None,
            partnership_type: None,
        }
    }

    pub fn page_size(mut self, page_size: u16) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page(mut self, page: u16) -> Self {
        self.page = Some(page);
        self
    }

    pub fn search(mut self, search: &str) -> Self {
        self.search = Some(search.to_string());
        self
    }

    pub fn sort_by(mut self, sort_by: AccountListSortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }

    pub fn billing_version(mut self, billing_version: AccountBillingVersion) -> Self {
        self.billing_version = Some(billing_version);
        self
    }

    pub fn partnership_type(mut self, partnership_type: AccountPartnershipType) -> Self {
        self.partnership_type = Some(partnership_type);
        self
    }

    pub async fn send(self) {
        let route = format!("/accounts");
    }
}
