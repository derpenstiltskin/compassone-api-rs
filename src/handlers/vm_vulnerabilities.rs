use crate::CompassOneClient;

pub struct VmVulnerabilitiesHandler<'co> {
    client: &'co CompassOneClient,
}

impl<'co> VmVulnerabilitiesHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self { client }
    }
}
