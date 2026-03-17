use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct CloudMdrGoogleHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> CloudMdrGoogleHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_onboarding(&self) -> CloudMdrGoogleGetOnboarding<'_, '_, '_> {
        CloudMdrGoogleGetOnboarding::new(self)
    }

    pub fn list_onboardings(&self) -> CloudMdrGoogleListOnboardingsBuilder<'_, '_, '_> {
        CloudMdrGoogleListOnboardingsBuilder::new(self)
    }
}
