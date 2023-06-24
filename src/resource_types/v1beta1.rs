use crate::actions::{ChartExtActionTarget, ChartExtActionUserDeploymentRole};
use crate::ui_schema::UiSchemaV0;
use crate::versions::{ChartExtKindResourceType, ChartExtVersionV1Beta1};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ChartExtResourceTypeV1Beta1 {
    pub api_version: ChartExtVersionV1Beta1,
    pub kind: ChartExtKindResourceType,
    pub key: String,
    pub spec: ChartExtResourceTypeV1Beta1Spec,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChartExtResourceTypeV1Beta1Spec {
    pub name_singular: String,
    pub name_plural: String,
    pub fontawesome_icon: String,
    #[serde(default)]
    pub global: bool,
    pub values_ui: UiSchemaV0,
    #[serde(default)]
    pub lifecycle: ChartExtResourceLifecycleV1Beta1,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChartExtResourceLifecycleV1Beta1 {
    pub create: Option<ChartExtResourceLifecycleActionV1Beta1>,
    pub update: Option<ChartExtResourceLifecycleActionV1Beta1>,
    pub delete: Option<ChartExtResourceLifecycleActionV1Beta1>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChartExtResourceLifecycleActionV1Beta1 {
    pub allowed_role: Option<ChartExtActionUserDeploymentRole>,
    pub target: Option<ChartExtActionTarget>,
}
