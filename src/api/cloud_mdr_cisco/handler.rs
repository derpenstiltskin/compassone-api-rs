use crate::CompassOneClient;

pub struct CloudMdrCisco<'co> {
    client: &'co CompassOneClient
}

impl<'co> CloudMdrCisco<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }
}
