use crate::CompassOneClient;
use serde::Serialize;

pub struct AccountHandler<'co> {
    client: &'co CompassOneClient,
}

impl<'co> AccountHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self { client }
    }

    pub fn get_account(&self, id: &str) -> AccountGetBuilder<'_, '_> {
        AccountGetBuilder::new(self)
    }

    pub fn list_accounts(&self) -> AccountListBuilder<'_, '_> {
        AccountListBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct AccountGetBuilder<'co, 'r> {
    #[serde(skip)]
    handler: &'r AccountHandler<'co>,
}

impl<'co, 'r> AccountGetBuilder<'co, 'r> {
    pub(crate) fn new(handler: &'r AccountHandler<'co>) -> Self {
        Self { handler }
    }
}

#[derive(Serialize)]
pub struct AccountListBuilder<'co, 'r> {
    #[serde(skip)]
    handler: &'r AccountHandler<'co>,
}

impl<'co, 'r> AccountListBuilder<'co, 'r> {
    pub(crate) fn new(handler: &'r AccountHandler<'co>) -> Self {
        Self { handler }
    }
}
