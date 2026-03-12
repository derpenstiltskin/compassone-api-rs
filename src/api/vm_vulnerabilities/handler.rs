use crate::CompassOneClient;

pub struct VmVulnerabilities<'co> {
    client: &'co CompassOneClient
}

impl<'co> VmVulnerabilities<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }
}
