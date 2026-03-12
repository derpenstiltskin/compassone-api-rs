use crate::CompassOneClient;

pub struct DetectionsHandler<'co> {
    client: &'co CompassOneClient
}

impl<'co> DetectionsHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }
}
