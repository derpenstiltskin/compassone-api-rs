use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct TenantHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> TenantHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_tenant(&self) -> TenantGetBuilder<'_, '_, '_> {
        TenantGetBuilder::new(self)
    }

    pub fn list_tenants(&self) -> TenantListBuilder<'_, '_, '_> {
        TenantListBuilder::new(self)
    }
}
