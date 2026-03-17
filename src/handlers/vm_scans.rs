use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct VmScansHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> VmScansHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn list_scans_and_schedules(&self) -> VmScansListScansAndSchedulesBuilder<'_, '_, '_> {
        VmScansListScansAndSchedulesBulder::new(self)
    }

    pub fn get_scan_statistics(&self) -> VmScansGetStatisticsBuilder<'_, '_, '_> {
        VmScansGetStatisticsBulder::new(self)
    }

    pub fn bulk_delete_scans_and_schedules(&self) -> VmScansBulkDeleteScansAndSchedulesBuilder<'_, '_, '_> {
        VmScansBulkDeleteScansAndSchedulesBuilder::new(self)
    }

    pub fn bulk_update_schedules(&self) -> VmScansBulkUpdateSchedulesBuilder<'_, '_, '_> {
        VmScansBulkUpdateSchedulesBuilder::new(self)
    }

    pub fn export_scans(&self) -> VmScansExportBuilder<'_, '_, '_> {
        VmScansExportBuilder::new(self)
    }

    pub fn get_scan(&self) -> VmScansGetBuilder<'_, '_, '_> {
        VmScansGetBuilder::new(self)
    }

    pub fn list_scans(&self) -> VmScansListBuilder<'_, '_, '_> {
        VmScansListBuilder::new(self)
    }

    pub fn create_scan(&self) -> VmScansCreateBuilder<'_, '_, '_> {
        VmScansCreateBuilder::new(self)
    }

    pub fn update_scan(&self) -> VmScansUpdateBuilder<'_, '_, '_> {
        VmScansUpdateBuilder::new(self)
    }

    pub fn delete_scan(&self) -> VmScansDeleteBuilder<'_, '_, '_> {
        VmScansDeleteBuilder::new(self)
    }

    pub fn cancel_scan(&self) -> VmScansCancelBuilder<'_, '_, '_> {
        VmScansCancelBuilder::new(self)
    }

    pub fn list_scan_cves(&self) -> VmScansListCvesBuilder<'_, '_, '_> {
        VmScansListCvesBuilder::new(self)
    }

    pub fn export_scan_cves(&self) -> VmScansExportCvesBuilder<'_, '_, '_> {
        VmScansExportCvesBuilder::new(self)
    }

    pub fn create_schedule(&self) -> VmScansCreateScheduleBuilder<'_, '_, '_> {
        VmScansCreateScheduleBuilder::new(self)
    }

    pub fn get_schedule(&self) -> VmScansGetScheduleBuilder<'_, '_, '_> {
        VmScansGetScheduleBuilder::new(self)
    }

    pub fn list_schedules(&self) -> VmScansListSchedulesBuilder<'_, '_, '_> {
        VmScansListSchedulesBuilder::new(self)
    }

    pub fn run_schedule(&self) -> VmScansRunScheduleBuilder<'_, '_, '_> {
        VmScansRunScheduleBuilder::new(self)
    }

    pub fn update_schedule(&self) -> VmScansUpdateScheduleBuilder<'_, '_, '_> {
        VmScansUpdateScheduleBuilder::new(self)
    }

    pub fn delete_schedule(&self) -> VmScansUpdateScheduleBuilder<'_, '_, '_> {
        VmScansUpdateScheduleBuilder::new(self)
    }
}
