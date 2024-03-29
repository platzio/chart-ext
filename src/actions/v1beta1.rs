use super::v0::ChartExtActionV0;
use crate::versions::{ChartExtKindAction, ChartExtVersionV1Beta1};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChartExtActionsV1Beta1(Vec<ChartExtActionV1Beta1>);

impl ChartExtActionsV1Beta1 {
    pub fn get_actions(&self) -> Vec<ChartExtActionV0> {
        self.0.iter().map(|x| x.spec.clone()).collect()
    }
    pub fn find(&self, action_id: &str) -> Option<&ChartExtActionV0> {
        self.0
            .iter()
            .find(|action| action.spec.id == action_id)
            .map(|action| &action.spec)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ChartExtActionV1Beta1 {
    pub api_version: ChartExtVersionV1Beta1,
    pub kind: ChartExtKindAction,
    pub spec: ChartExtActionV0,
}
