use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct GoogleOnboardingConfigModel {
    pub redirect_path: String,
    pub domain: String,
}
