use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum HelmChartV2 {
    #[default]
    #[serde(rename = "v2")]
    Value,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ChartExtVersionV1Beta1 {
    #[default]
    #[serde(rename = "platz.io/v1beta1")]
    Value,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ChartExtVersionV1Beta2 {
    #[default]
    #[serde(rename = "platz.io/v1beta2")]
    Value,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ChartExtKindValuesUi {
    #[default]
    #[serde(rename = "ValuesUi")]
    Value,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ChartExtKindAction {
    #[default]
    #[serde(rename = "Action")]
    Value,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ChartExtKindFeatures {
    #[default]
    #[serde(rename = "Features")]
    Value,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ChartExtKindResourceType {
    #[default]
    #[serde(rename = "ResourceType")]
    Value,
}
