use crate::CompassOneClient;

pub struct VmExternalHandler<'co> {
    client: &'co CompassOneClient
}

impl<'co> VmExternalHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }
}
