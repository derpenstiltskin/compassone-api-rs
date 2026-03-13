use crate::CompassOneClient;

pub struct CloudMdrM365<'co> {
    client: &'co CompassOneClient,
}

impl<'co> CloudMdrM365<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self { client }
    }
}
