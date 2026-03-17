use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct CloudMdrM365Handler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> CloudMdrM365Handler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_connection(&self) -> CloudMdr365GetConnectionBuilder<'_, '_, '_> {
        CloudMdrM365GetConnectionBuilder::new(self)
    }

    pub fn list_connections_by_tenant_id(&self) -> CloudMdr365ListConnectionsByTenantIdBuilder<'_, '_, '_> {
        CloudMdr365ListConnectionsByTenantIdBuilder::new(self)
    }

    pub fn list_approved_countries(&self) -> CloudMdrM365ListApprovedCountriesBuilder<'_, '_, '_> {
        CloudMdr365ListApprovedCountriesBuilder::new(self)
    }

    pub fn approve_country(&self) -> CloudMdrM365ApproveCountryBuilder<'_, '_, '_> {
        CloudMdrM365ApproveCountryBuilder::new(self)
    }

    pub fn remove_approved_country(&self) -> CloudMdrM365RemoveApprovedCountryBuilder<'_, '_, '_> {
        CloudMdrM365RemoveApprovedCountryBuilder::new(self)
    }

    pub fn list_users(&self) -> CloudMdrM365ListUsersBuilder<'_, '_, '_> {
        CloudMdrM365ListUsersBuilder::new(self)
    }

    pub fn approve_country_for_user(&self) -> CloudMdrM365ApproveCountryForUserBuilder<'_, '_, '_> {
        CloudMdrM365ApproveCountryForUserBuilder::new(self)
    }

    pub fn remove_approved_country_for_user(&self) -> CloudMdrM365RemoveApprovedCountryForUserBuilder<'_, '_, '_> {
        CloudMdrM365RemoveApprovedCountryForUserBuilder::new(self)
    }

    pub fn list_approved_countries_for_user(&self) -> CloudMdrM365ListApprovedCountriesForUserBuilder<'_, '_, '_> {
        CloudMdrM365ListApprovedCountriesForUserBuilder::new(self)
    }
}
