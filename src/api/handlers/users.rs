use crate::CompassOneClient;

pub struct UsersHandler<'co> {
    client: &'co CompassOneClient,
}

impl<'co> UsersHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self { client }
    }
}
