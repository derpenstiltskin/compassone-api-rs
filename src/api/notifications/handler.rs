use crate::CompassOneClient;

pub struct NotificationsHandler<'co> {
    client: &'co CompassOneClient
}

impl<'co> NotificationsHandler<'co> {
    pub(crate) fn new(client: &'co CompassOneClient) -> Self {
        Self {
            client
        }
    }
}
