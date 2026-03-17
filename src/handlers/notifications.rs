use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct NotificationsHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> NotificationsHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn list_channels(&self) -> NotificationsListChannelsBuilder<'_, '_, '_> {
        NotificationsListChannelsBuilder::new(self)
    }

    pub fn get_email_channel(&self) -> NotificationsGetEmailChannel<'_, '_, '_> {
        NotificationsGetEmailChannelBuilder::new(self)
    }

    pub fn list_email_channels(&self) -> NotificationsListEmailChannels<'_, '_, '_> {
        NotificationsListEmailChannelsBuilder::new(self)
    }

    pub fn create_email_channel(&self) -> NotificationsCreateEmailChannelBuilder<'_, '_, '_> {
        NotificationsCreateEmailChannelBuilder::new(self)
    }

    pub fn update_email_channel(&self) -> NotificationsUpdateEmailChannelBuilder<'_, '_, '_> {
        NotificationsUpdateEmailChannelBuilder::new(self)
    }

    pub fn delete_email_channel(&self) -> NotificationsDeleteEmailChannelBuilder<'_, '_, '_> {
        NotificationsDeleteEmailChannelBuilder::new(self)
    }

    pub fn duplicate_email_channel(&self) -> NotificationsDuplicateEmailChannelBuilder<'_, '_, '_> {
        NotificationsDuplicateEmailChannelBuilder::new(self)
    }

    pub fn test_email_channel(&self) -> NotificationsTestEmailChannelBuilder<'_, '_, '_> {
        NotificationsTestEmailChannelBuilder::new(self)
    }

    pub fn get_webhook_channel(&self) -> NotificationsGetWebhookChannelBuilder<'_, '_, '_> {
        NotificationsGetWebhookChannelBuilder::new(self)
    }

    pub fn list_webhook_channels(&self) -> NotificationsListWebhookChannelsBuilder<'_, '_, '_> {
        NotificationsListWebhookChannelsBuilder::new(self)
    }

    pub fn create_webhook_channel(&self) -> NotificationsCreateWebhookChannelBuilder<'_, '_, '_> {
        NotificationsCreateWebhookChannelBuilder::new(self)
    }

    pub fn update_webhook_channel(&self) -> NotificationsUpdateWebhookChannelBuilder<'_, '_, '_> {
        NotificationsUpdateWebhookChannelBuilder::new(self)
    }

    pub fn delete_webhook_channel(&self) -> NotificationsDeleteWebhookChannelBuildeR<'_, '_, '_> {
        NotificationsDeleteWebhookChannelBuilder::new(self)
    }

    pub fn duplicate_webhook_channel(&self) -> NotificationsDuplicateWebhookChannelBuilder<'_, '_, '_> {
        NotificationsDuplicateWebhookChannelBuilder::new(self)
    }

    pub fn test_webhook_channel(&self) -> NotificationsTestWebhookChannelBuilder<'_, '_, '_> {
        NotificationsTestWebhookChannelBuilder::new(self)
    }

    pub fn block_tenant(&self) -> NotificationsBlockTenantBuilder<'_, '_, '_> {
        NotificationsBlockTenantBuilder::new(self)
    }

    pub fn unblock_tenant(&self) -> NotificationsUnblockTenantBuilder<'_, '_, '_> {
        NotificationsUnblockTenantBuilder::new(self)
    }

    pub fn check_if_tenant_blocked(&self) -> NotificationsCheckIfTenantBlockedBuilder<'_, '_, '_> {
        NotificationsCheckIfTenantBlockedBuilder::new(self)
    }
}
