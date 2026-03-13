use crate::CompassOneClient;

pub struct CloudMdr<'co> {
    client: &'co CompassOneClient,
}

impl<'co> CloudMdr<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self { client }
    }
}
