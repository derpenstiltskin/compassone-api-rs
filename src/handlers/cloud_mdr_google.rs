use crate::CompassOneClient;

pub struct CloudMdrGoogle<'co> {
    client: &'co CompassOneClient,
}

impl<'co> CloudMdrGoogle<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self { client }
    }
}
