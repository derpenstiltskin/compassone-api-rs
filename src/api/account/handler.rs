use crate::{CompassOneClient};

use super::builder::{AccountGetBuilder, AccountListBuilder};

pub struct AccountHandler<'co> {
    client: &'co CompassOneClient
}

impl<'co> AccountHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }

    pub fn get_account(&self, id: &str) -> AccountGetBuilder<'_, '_> {
        AccountGetBuilder::new(self)
    }

    pub fn list_accounts(&self) -> AccountListBuilder<'_, '_> {
        AccountListBuilder::new(self)
    }
}
