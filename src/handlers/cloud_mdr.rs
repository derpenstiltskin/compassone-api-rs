use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct CloudMdrHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> CloudMdrHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_country(&self) -> CloudMdrGetCountryBuilder<'_, '_, '_> {
        CloudMdrGetCountryBuilder::new(self)
    }

    pub fn list_countries(&self) -> CloudMdrListCountriesBuilder<'_, '_, '_> {
        CloudMdrListCountriesBuilder::new(self)
    }

    pub fn list_approved_countries(&self) -> CloudMdrListApprovedCountriesBuilder<'_, '_, '_> {
        CloudMdrListApprovedCountriesBuilder::new(self)
    }

    pub fn approve_country(&self) -> CloudMdrApproveCountryBuilder<'_, '_, '_> {
        CloudMdrApproveCountryBuilder::new(self)
    }

    pub fn remove_approved_country(&self) -> CloudMdrRemoveApprovedCountryBuilder<'_, '_, '_> {
        CloudMdrRemoveApprovedCountryBuilder::new(self)
    }

    pub fn list_users(&self) -> CloudrMdrListUsersBuilder<'_, '_, '_> {
        CloudMdrListUsersBuilder::new(self)
    }

    pub fn list_approved_user_countries(&self) -> CloudMdrListApprovedUserCountriesBuilder<'_, '_, '_> {
        CloudMdrListApprovedUserCountries(self)
    }

    pub fn approve_country_for_user(&self) -> CloudMdrApproveCountryForUserBuilder<'_, '_, '_> {
        CloudMdrApproveCountryForUserBuilder::new(self)
    }

    pub fn remove_approved_country_for_user(&self) -> CloudMdrRemoveApprovedCountryForUser<'_, '_, '_> {
        CloudMdrRemoveApprovedCountryForUser::new(self)
    }
}
