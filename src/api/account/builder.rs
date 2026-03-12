use serde::Serialize;

use super::handler::AccountHandler;

#[derive(Serialize)]
pub struct AccountGetBuilder<'co, 'r> {
    #[serde(skip)]
    handler: &'r AccountHandler<'co>
}

impl<'co, 'r> AccountGetBuilder<'co, 'r> {
    pub(crate) fn new(handler: &'r AccountHandler<'co>) -> Self {
        Self {
            handler
        }
    }
}

#[derive(Serialize)]
pub struct AccountListBuilder<'co, 'r> {
    #[serde(skip)]
    handler: &'r AccountHandler<'co>
}

impl<'co, 'r> AccountListBuilder<'co, 'r> {
    pub(crate) fn new(handler: &'r AccountHandler<'co>) -> Self {
        Self {
            handler
        }
    }
}
