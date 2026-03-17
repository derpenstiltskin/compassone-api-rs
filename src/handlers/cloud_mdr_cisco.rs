use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct CloudMdrCiscoHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> CloudMdrCiscoHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_onboarding(&self) -> CloudMdrCiscoGetOnboardingBuilder<'_, '_, '_> {
        CloudMdrCiscoGetOnboardingBuilder::new(self)
    }

    pub fn list_onboardings(&self) -> CloudMdrCiscoListOnboardingsBuilder<'_, '_, '_> {
        CloudMdrCiscoListOnboardingsBuilder::new(self)
    }

    pub fn create_onboarding(&self) -> CloudMdrCiscoCreateOnboardingBuilder<'_, '_, '_> {
        CloudMdrCiscoCreateOnboardingBuilder::new(self)
    }

    pub fn delete_onboarding(&self) -> CloudMdrCiscoDeleteOnboardingBuilder<'_, '_, '_> {
        CloudMdrCiscoDeleteOnboardingBuilder::new(self)
    }

    pub fn sync_users(&self) -> CloudMdrCiscoSyncUsersBuilder<'_, '_, '_> {
        CloudMdrCiscoSyncUsersBuilder::new(self)
    }

    pub fn complete_onboarding(&self) -> CloudMdrCiscoCompleteOnboardingBuilder<'_, '_, '_> {
        CloudMdrCiscoCompleteOnboardingBuilder::new(self)
    }
}
