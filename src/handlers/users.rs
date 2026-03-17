use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct UsersHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> UsersHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn list_users(&self) -> UserListBuilder<'_, '_, '_> {
        UserListBuilder::new(self)
    }

    pub fn delete_user(&self) -> UserDeleteBuilder<'_, '_, '_> {
        UserDeleteBuilder::new(self)
    }

    pub fn reset_password(&self) -> UserResetPasswordBuilder<'_, '_, '_> {
        UserResetPasswordBuilder::new(self)
    }

    pub fn invite_user_to_account(&self) -> UserInviteToAccountBuilder<'_, '_, '_> {
        UserInviteToAccountBuilder::new(self)
    }

    pub fn list_users_for_account(&self) -> UserListForAccountBuilder<'_, '_, '_> {
        UserListForAccountBuilder::new(self)
    }

    pub fn delete_account_user(&self) -> UserDeleteAccountUserBuilder<'_, '_, '_> {
        UserDeleteAccountUserBuilder::new(self)
    }

    pub fn update_account_user(&self) -> UserUpdateAccountUserBuilder<'_, '_, '_> {
        UserUpdateAccountUserBuilder::new(self)
    }

    pub fn list_users_for_tenant(&self) -> UserListForTenantBuilder<'_, '_, '_> {
        UserListForTenantBuilder::new(self)
    }

    pub fn assign_users_to_tenant(&self) -> UserAssignToTenantBuilder<'_, '_, '_> {
        UserAssignToTenantBuilder::new(self)
    }

    pub fn unassign_users_from_tenant(&self) -> UserUnassignFromTenantBuilder<'_, '_, '_> {
        UserUnassignFromTenantBuilder::new(self)
    }

    pub fn list_users_assigned_to_tenant(&self) -> UserListUnassignedToTenantBuilder<'_, '_, '_> {
        UserListUnassignedToTenantBuilder::new(self)
    }
}
