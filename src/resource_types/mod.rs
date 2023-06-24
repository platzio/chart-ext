mod v1beta1;

pub use self::v1beta1::{
    ChartExtResourceLifecycleActionV1Beta1, ChartExtResourceLifecycleV1Beta1,
    ChartExtResourceTypeV1Beta1, ChartExtResourceTypeV1Beta1Spec,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChartExtResourceTypes(pub Vec<ChartExtResourceType>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(untagged)]
pub enum ChartExtResourceType {
    V1Beta1(ChartExtResourceTypeV1Beta1),
}
