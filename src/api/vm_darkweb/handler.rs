use crate::CompassOneClient;

pub struct VmDarkwebHandler<'co> {
    client: &'co CompassOneClient
}

impl<'co> VmDarkwebHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }
}
