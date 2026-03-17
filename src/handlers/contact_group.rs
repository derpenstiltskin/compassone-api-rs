use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct ContactGroupHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> ContactGroupHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_contact_group(&self) -> ContactGroupGetBuilder<'_, '_, '_> {
        ContactGroupGetBuilder::new(self)
    }

    pub fn list_contact_groups(&self) -> ContactGroupListBuilder<'_, '_, '_> {
        ContactGroupListBuilder::new(self)
    }

    pub fn create_contact_group(&self) -> ContactGroupCreateBuilder<'_, '_, '_> {
        ContactGroupCreateBuilder::new(self)
    }

    pub fn update_contact_group(&self) -> ContactGroupUpdateBuilder<'_, '_, '_> {
        ContactGroupUpdateBuilder::new(self)
    }

    pub fn delete_contact_group(&self) -> ContactGroupDeleteBuilder<'_, '_, '_> {
        ContactGroupDeleteBuilder::new(self)
    }

    pub fn list_tenants(&self) -> ContactGroupListTenantsBuilder<'_, '_, '_> {
        ContactGroupListTenantsBuilder::new(self)
    }

    pub fn add_tenants(&self) -> ContactGroupAddTenantsBuilder<'_, '_, '_> {
        ContactGroupAddTenantsBuilder::new(self)
    }

    pub fn list_tenants_unassigned(&self) -> ContactGroupListTenantsUnassignedBuilder<'_, '_, '_> {
        ContactGroupListTenantsUnassignedBuilder::new(self)
    }

    pub fn list_members(&self) -> ContactGroupListMembersBuilder<'_, '_, '_> {
        ContactGroupListMembersBuilders::new(self)
    }

    pub fn add_members(&self) -> ContactGroupAddMembersBuilder<'_, '_, '_> {
        ContactGroupAddMembersBuilder::new(self)
    }

    pub fn get_member(&self) -> ContactGroupGetMemberBuilder<'_, '_, '_> {
        ContactGroupGetMemberBuilder::new(self)
    }

    pub fn delete_member(&self) -> ContactGroupDeleteMemberBuilder<'_, '_, '_> {
        ContactGroupDeleteMember::new(self)
    }
}
