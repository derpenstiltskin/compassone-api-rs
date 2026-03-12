use crate::CompassOneClient;

pub struct TenantHandler<'co> {
    client: &'co CompassOneClient
}

impl<'co> TenantHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }
}
