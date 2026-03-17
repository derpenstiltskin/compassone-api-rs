use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
    models::{
        AssetClass,
        AssetRelationshipDirection,
    },
};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AssetListSortBy {
    #[serde(rename(serialize = "accountId", deserialize = "AccountId"))]
    AccountId,
    #[serde(rename(serialize = "assetClass", deserialize = "AssetClass"))]
    AssetClass,
    #[serde(rename(serialize = "classification", deserialize = "Classification"))]
    Classification,
    #[serde(rename(serialize = "createdBy", deserialize = "CreatedBy"))]
    CreatedBy,
    #[serde(rename(serialize = "createdOn", deserialize = "CreatedOn"))]
    CreatedOn,
    #[serde(rename(serialize = "criticality", deserialize = "Criticality"))]
    Criticality,
    #[serde(rename(serialize = "deletedBy", deserialize = "DeletedBy"))]
    DeletedBy,
    #[serde(rename(serialize = "deletedOn", deserialize = "DeletedOn"))]
    DeletedOn,
    #[serde(rename(serialize = "description", deserialize = "Description"))]
    Description,
    #[serde(rename(serialize = "displayName", deserialize = "DisplayName"))]
    DisplayName,
    #[serde(rename(serialize = "foundBy", deserialize = "FoundBy"))]
    FoundBy,
    #[serde(rename(serialize = "foundOn", deserialize = "FoundOn"))]
    FoundOn,
    #[serde(rename(serialize = "id", deserialize = "Id"))]
    Id,
    #[serde(rename(serialize = "lastSeenOn", deserialize = "LastSeenOn"))]
    LastSeenOn,
    #[serde(rename(serialize = "name", deserialize = "Name"))]
    Name,
    #[serde(rename(serialize = "status", deserialize = "Status"))]
    Status,
    #[serde(rename(serialize = "summary", deserialize = "Summary"))]
    Summary,
    #[serde(rename(serialize = "tenantId", deserialize = "TenantId"))]
    TenantId,
    #[serde(rename(serialize = "type", deserialize = "Type"))]
    Type,
    #[serde(rename(serialize = "updatedBy", deserialize = "UpdatedBy"))]
    UpdatedBy,
    #[serde(rename(serialize = "updatedOn", deserialize = "UpdatedOn"))]
    UpdatedOn,
    #[serde(rename(serialize = "agentLastSeenOn", deserialize = "AgentLastSeenOn"))]
    AgentLastSeenOn,
    #[serde(rename(serialize = "agentDeactivatedOn", deserialize = "AgentDeactivatedOn"))]
    AgentDeactivatedOn,
    #[serde(rename(serialize = "lastLoginOn", deserialize = "LastLoginOn"))]
    LastLoginOn,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum AssetRelationshipListSortBy {
    #[serde(rename(serialize = "created_on", deserialize = "CreatedOn"))]
    CreatedOn,
}

pub struct AssetHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> AssetHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_asset(&self, id: &str) -> AssetGetBuilder<'_, '_, '_> {
        AssetGetBuilder::new(self, id)
    }

    pub fn list_assets(&self, class: AssetClass) -> AssetListBuilder<'_, '_, '_> {
        AssetListBuilder::new(self, class)
    }

    pub fn list_asset_relationships(&self, id: &str, class: AssetClass, direction: AssetRelationshipDirection) -> AssetListRelationshipsBuilder<'_, '_, '_> {
        AssetListRelationshipsBuilder::new(self, id, class, direction)
    }
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AssetGetBuilder<'c, 'h, 'r> {
    #[serde(skip)]
    handler: &'r AssetHandler<'c, 'h>,
    #[serde(skip)]
    id: String,
}

impl<'c, 'h, 'r> AssetGetBuilder<'c, 'h, 'r> {
    pub(crate) fn new(handler: &'r AssetHandler<'c, 'h>, id: &str) -> Self {
        Self {
            handler,
            id: id.to_string(),
        }
    }

    pub async fn send(self) {
        let route = format!("/assets/{}", self.id);
    }
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AssetListBuilder<'c, 'h, 'r> {
    #[serde(skip)]
    handler: &'r AssetHandler<'c, 'h>,
    class: AssetClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    found_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_seen_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decommissioned: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wd_status: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<AssetListSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<GenericListSortOrder>,
}

impl<'c, 'h, 'r> AssetListBuilder<'c, 'h, 'r> {
    pub(crate) fn new(handler: &'r AssetHandler<'c, 'h>, class: AssetClass) -> Self {
        Self {
            handler,
            class,
            page_size: None,
            page: None,
            search: None,
            filter: None,
            with_deleted: None,
            sources: None,
            platform: None,
            r#type: None,
            found_on: None,
            last_seen_on: None,
            decommissioned: None,
            wd_status: None,
            sort_by: None,
            sort_order: None,
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

    pub fn filter(mut self, filter: &str) -> Self {
        self.filter = Some(filter.to_string());
        self
    }

    pub fn with_deleted(mut self, with_deleted: bool) -> Self {
        self.with_deleted = Some(with_deleted);
        self
    }

    pub fn sources(mut self, sources: Vec<String>) -> Self {
        self.sources = Some(sources);
        self
    }

    pub fn platform(mut self, platform: Vec<String>) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn r#type(mut self, r#type: Vec<String>) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn found_on(mut self, found_on: Vec<String>) -> Self {
        self.found_on = Some(found_on);
        self
    }

    pub fn last_seen_on(mut self, last_seen_on: Vec<String>) -> Self {
        self.last_seen_on = Some(last_seen_on);
        self
    }

    pub fn decommissioned(mut self, decommissioned: Vec<String>) -> Self {
        self.decommissioned = Some(decommissioned);
        self
    }

    pub fn wd_status(mut self, wd_status: Vec<String>) -> Self {
        self.wd_status = Some(wd_status);
        self
    }

    pub fn sort_by(mut self, sort_by: AssetListSortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }

    pub fn sort_order(mut self, sort_order: GenericListSortOrder) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    pub async fn send(self) {
        let route = format!("/assets");
    }
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AssetListRelationshipsBuilder<'c, 'h, 'r> {
    #[serde(skip)]
    handler: &'r AssetHandler<'c, 'h>,
    #[serde(skip)]
    id: String,
    #[serde(skip)]
    class: AssetClass,
    #[serde(skip)]
    direction: AssetRelationshipDirection,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<AssetRelationshipListSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<GenericListSortOrder>,
}

impl<'c, 'h, 'r> AssetListRelationshipsBuilder<'c, 'h, 'r> {
    pub(crate) fn new(handler: &'r AssetHandler<'c, 'h>, id: &str, class: AssetClass, direction: AssetRelationshipDirection) -> Self {
        Self {
            handler,
            id: id.to_string(),
            class,
            direction,
            page_size: None,
            page: None,
            with_deleted: None,
            sort_by: None,
            sort_order: None
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

    pub fn with_deleted(mut self, with_deleted: bool) -> Self {
        self.with_deleted = Some(with_deleted);
        self
    }

    pub fn sort_by(mut self, sort_by: AssetRelationshipListSortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }

    pub fn sort_order(mut self, sort_order: GenericListSortOrder) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    pub async fn send(self) {
        let route = format!("/asset/{}/relationships", self.id);
    }
}
