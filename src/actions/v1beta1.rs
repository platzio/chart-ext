use super::v0::ChartExtActionV0;
use crate::versions::{ChartExtKindAction, ChartExtVersionV1Beta1};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(transparent)]
pub struct ChartExtActionsV1Beta1 {
    pub actions: Vec<ChartExtActionV1Beta1>,
}

impl ChartExtActionsV1Beta1 {
    pub fn find(&self, action_id: &str) -> Option<&ChartExtActionV0> {
        self.actions
            .iter()
            .find(|action| action.spec.id == action_id)
            .map(|action| &action.spec)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ChartExtActionV1Beta1 {
    pub api_version: ChartExtVersionV1Beta1,
    pub kind: ChartExtKindAction,
    pub spec: ChartExtActionV0,
}
