use crate::CompassOneClient;

pub struct CollectionHandler<'co> {
    client: &'co CompassOneClient
}

impl<'co> CollectionHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }
}
