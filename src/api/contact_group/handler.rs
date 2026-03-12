use crate::CompassOneClient;

pub struct ContactGroupHandler<'co> {
    client: &'co CompassOneClient
}

impl<'co> ContactGroupHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }
}
