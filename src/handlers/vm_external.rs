use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct VmExternalHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> VmExternalHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn list_exposures(&self) -> VmExternalListExposuresBuilder<'_, '_, '_> {
        VmExternalListExposuresBuilder::new(self)
    }

    pub fn get_report(&self) -> VmExternalGetReportBuilder<'_, '_, '_> {
        VmExternalGetReportBuilder::new(self)
    }
}
