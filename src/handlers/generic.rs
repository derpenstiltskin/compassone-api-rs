use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
#[non_exhaustive]
pub enum GenericListSortOrder {
    #[serde(rename(serialize = "ASC", deserialize = "Asc"))]
    Asc,
    #[serde(rename(serialize = "DESC", deserialize = "Desc"))]
    Desc,
}
