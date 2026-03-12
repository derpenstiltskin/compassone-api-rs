use crate::CompassOneClient;

pub struct AssetHandler<'co> {
    client: &'co CompassOneClient
}

impl<'co> AssetHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }
}
