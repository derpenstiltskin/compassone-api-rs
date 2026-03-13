use crate::CompassOneClient;

pub struct VmScansHandler<'co> {
    client: &'co CompassOneClient,
}

impl<'co> VmScansHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self { client }
    }
}
