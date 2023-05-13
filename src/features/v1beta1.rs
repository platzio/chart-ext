use super::v0::ChartExtFeaturesV0;
use crate::versions::{ChartExtKindFeatures, ChartExtVersionV1Beta1};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ChartExtFeaturesV1Beta1 {
    pub api_version: ChartExtVersionV1Beta1,
    pub kind: ChartExtKindFeatures,
    pub spec: ChartExtFeaturesV0,
}
