pub mod v0;
pub mod v1beta1;

use self::v0::ChartExtActionsV0;
use self::v1beta1::ChartExtActionsV1Beta1;
use serde::{Deserialize, Serialize};
pub use v0::{ChartExtActionEndpoint, ChartExtActionTarget, ChartExtActionTargetResolver};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(untagged)]
pub enum ChartExtActions {
    V1Beta1(ChartExtActionsV1Beta1),
    V0(ChartExtActionsV0),
}

impl ChartExtActions {
    pub fn find(&self, action_id: &str) -> Option<&v0::ChartExtActionV0> {
        match self {
            Self::V1Beta1(v1) => v1.find(action_id),
            Self::V0(v0) => v0.find(action_id),
        }
    }
}
