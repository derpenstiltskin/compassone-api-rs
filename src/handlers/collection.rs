use serde::{Deserialize, Serialize};

use crate::{
    CompassOneClient,
    handlers::GenericListSortOrder,
};

pub struct CollectionHandler<'c, 'h> {
    client: &'h CompassOneClient<'c>,
}

impl<'c, 'h> CollectionHandler<'c, 'h> {
    pub(crate) fn new(client: &'h CompassOneClient<'c>) -> Self {
        Self { client }
    }

    pub fn get_collection(&self) -> CollectionGetBuilder<'_, '_, '_> {
        CollectionGetBuilder::new(self)
    }

    pub fn list_collections(&self) -> CollectionListBuilder<'_, '_, '_> {
        CollectionListBuilder::new(self)
    }

    pub fn create_collection(&self) -> CollectionCreateBuilder<'_, '_, '_> {
        CollectionCreateBuilder::new(self)
    }

    pub fn update_collection(&self) -> CollectionUpdateBuilder<'_, '_, '_> {
        CollectionUpdateBuilder::new(self)
    }

    pub fn delete_collection(&self) -> CollectionDeleteBuilder<'_, '_, '_> {
        CollectionDeleteBuilder::new(self)
    }
}
