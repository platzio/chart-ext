mod v0;
mod v1beta1;

pub use self::v0::{
    ChartExtActionEndpoint, ChartExtActionMethod, ChartExtActionTarget,
    ChartExtActionTargetResolver, ChartExtActionUserDeploymentRole, ChartExtActionV0,
    ChartExtActionsV0,
};
pub use self::v1beta1::{ChartExtActionV1Beta1, ChartExtActionsV1Beta1};
use serde::{Deserialize, Serialize};

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
