use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct VmVulnerabilitiesHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> VmVulnerabilitiesHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_cve(&self) -> VmVulnerabilitiesGetCveBuilder<'_, '_, '_> {
        VmVulnerabilitiesGetCveBuilder::new(self)
    }

    pub fn list_cve_references(&self) -> VmVulnerabilitiesListCveReferencesBuilder<'_, '_, '_> {
        VmVulnerabilitiesListCveReferencesBuilder::new(self)
    }

    pub fn get_vulnerability(&self) -> VmVulnerabilitiesGetBuilder<'_, '_, '_> {
        VmVulnerabilitiesGetBuilder::new(self)
    }

    pub fn list_vulnerabilities(&self) -> VmVulnerabilitiesListBuilder<'_, '_, '_> {
        VmVulnerabilitiesListBuilder::new(self)
    }

    pub fn list_vulnerability_assets(&self) -> VmVulnerabilitiesListAssetsBuilder<'_, '_, '_> {
        VmVulnerabilitiesListAssetsBuilder::new(self)
    }

    pub fn update_vulnerability_status_for_devices(&self) -> VmVulnerabilitiesUpdateStatusForDevicesBuilder<'_, '_, '_> {
        VmVulnerabilitiesUpdateStatusForDevices::new(self)
    }

    pub fn delete_vulnerabilities_for_devices(&self) -> VmVulnerabilitiesDeleteForDevicesBuilder<'_, '_, '_> {
        VmVulnerabilitiesDeleteForDevicesBuilder::new(self)
    }

    pub fn get_severity_statistics(&self) -> VmVulnerabilitiesGetSeverityStatisticsBuilder<'_, '_, '_> {
        VmVulnerabilitiesGetSeverityStatisticsBuilder::new(self)
    }

    pub fn get_tenant_statistics(&self) -> VmVulnerabilitiesGetTenantStatisticsBuilder<'_, '_, '_> {
        VmVulnerabilitiesGetTenantStatisticsBuilder::new(self)
    }

    pub fn bulk_delete_vulnerabilities_for_devices(&self) -> VmVulnerabilitiesBulkDeleteForDevicesBuilder<'_, '_, '_> {
        VmVulnerabilitiesBulkDeleteForDevicesBuilder::new(self)
    }

    pub fn bulk_update_vulnerabilities_for_devices(&self) -> VmVulnerabilitiesBulkUpdateForDevicesBuilder<'_, '_, '_> {
        VmVulnerabilitiesBulkUpdateForDevicesBuilder::new(self)
    }

    pub fn bulk_delete_vulnerabilities(&self) -> VmVulnerabilitiesBulkDeleteBuilder<'_, '_, '_> {
        VmVulnerabilitiesBulkDelete::new(self)
    }

    pub fn bulk_update_vulnerabilities(&self) -> VmVulnerabilitiesBulkUpdateBuilder<'_, '_, '_> {
        VmVulnerabilitiesBulkUpdate::new(self)
    }

    pub fn export_vulnerabilities(&self) -> VmVulnerabilitiesExportBuilder<'_, '_, '_> {
        VmVulnerabilitiesExportBuilder::new(self)
    }

    pub fn export_vulnerability_assets(&self) -> VmVulnerabilitiesExportAssetsBuilder<'_, '_, '_> {
        VmVulnerabilitiesExportAssetsBuilder::new(self)
    }
}
