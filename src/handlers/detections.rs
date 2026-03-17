use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct DetectionsHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> DetectionsHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_alert_group(&self) -> DetectionsGetAlertGroupBuilder<'_, '_, '_> {
        DetectionsGetAlertGroupBuilder::new(self)
    }

    pub fn list_alert_groups(&self) -> DetectionsListAlertGroupsBuilder<'_, '_, '_> {
        DetectionsListAlertGroupsBuilder::new(self)
    }

    pub fn list_alert_groups_by_week(&self) -> DetectionsListAlertGroupsByWeekBuilder<'_, '_, '_> {
        DetectionsListAlertGroupsByWeekBuilder::new(self)
    }

    pub fn get_alert_group_count(&self) -> DetectionsGetAlertGroupCountBuilder<'_, '_, '_> {
        DetectionsGetAlertGroupCountBuilder::new(self)
    }

    pub fn get_top_detections_by_entity(&self) -> DetectionsGetTopByEntityBuilder<'_, '_, '_> {
        DetectionsGetTopByEntityBuilder::new(self)
    }

    pub fn get_top_detections_by_threat(&self) -> DetectionsGetTopByThreatBuilder<'_, '_, '_> {
        DetectionsGetTopByThreatBuilder::new(self)
    }

    pub fn list_alerts_by_alert_group(&self) -> DetectionsListAlertsByAlertGroup<'_, '_, '_> {
        DetectionsListAlertsByAlertGroupBuilder::new(self)
    }
}
