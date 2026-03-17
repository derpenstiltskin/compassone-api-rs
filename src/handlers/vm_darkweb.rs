use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct VmDarkwebHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> VmDarkwebHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn list_exposures(&self) -> VmDarkwebListExposuresBuilder<'_, '_, '_> {
        VmDarkwebListExposuresBuilder::new(self)
    }

    pub fn get_report(&self) -> VmDarkwebGetReportBuilder<'_, '_, '_> {
        VmDarkwebGetReportBuilder::new(self)
    }
}
