mod v0;
mod v1beta1;
mod v1beta2;

pub use self::v0::{ChartExtCardinality, ChartExtFeaturesV0, ChartExtStatusFeature};
pub use self::v1beta1::ChartExtFeaturesV1Beta1;
pub use self::v1beta2::{
    ChartExtDeploymentDisplay, ChartExtDeploymentDisplayIcon, ChartExtDeploymentDisplayName,
    ChartExtDeploymentDisplayNameInputField, ChartExtFeaturesSpec, ChartExtFeaturesV1Beta2,
    ChartExtIngress, ChartExtIngressHostnameFormat,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(untagged)]
pub enum ChartExtFeatures {
    V1Beta2(ChartExtFeaturesV1Beta2),
    V1Beta1(ChartExtFeaturesV1Beta1),
    V0(ChartExtFeaturesV0),
}

impl Default for ChartExtFeatures {
    fn default() -> Self {
        Self::V1Beta2(Default::default())
    }
}

impl ChartExtFeatures {
    pub fn ingress(&self) -> v1beta2::ChartExtIngress {
        match self {
            Self::V1Beta2(features) => features.spec.ingress.clone(),
            Self::V1Beta1(features) => features.spec.standard_ingress.into(),
            Self::V0(features) => features.standard_ingress.into(),
        }
    }

    pub fn status(&self) -> Option<&v0::ChartExtStatusFeature> {
        match self {
            Self::V1Beta2(features) => features.spec.status.as_ref(),
            Self::V1Beta1(features) => features.spec.status.as_ref(),
            Self::V0(features) => features.status.as_ref(),
        }
    }

    pub fn cardinality(&self) -> &v0::ChartExtCardinality {
        match self {
            Self::V1Beta2(features) => &features.spec.cardinality,
            Self::V1Beta1(features) => &features.spec.cardinality,
            Self::V0(features) => &features.cardinality,
        }
    }

    pub fn reinstall_dependencies(&self) -> bool {
        match self {
            Self::V1Beta2(features) => features.spec.reinstall_dependencies,
            Self::V1Beta1(features) => features.spec.reinstall_dependencies,
            Self::V0(features) => features.reinstall_dependencies,
        }
    }

    pub fn node_selector_paths(&self) -> &Vec<Vec<String>> {
        match self {
            Self::V1Beta2(features) => &features.spec.node_selector_paths,
            Self::V1Beta1(features) => &features.spec.node_selector_paths,
            Self::V0(features) => &features.node_selector_paths,
        }
    }

    pub fn tolerations_paths(&self) -> &Vec<Vec<String>> {
        match self {
            Self::V1Beta2(features) => &features.spec.tolerations_paths,
            Self::V1Beta1(features) => &features.spec.tolerations_paths,
            Self::V0(features) => &features.tolerations_paths,
        }
    }

    pub fn display(&self) -> v1beta2::ChartExtDeploymentDisplay {
        match self {
            Self::V1Beta2(features) => features.spec.display.clone(),
            Self::V1Beta1(_) => Default::default(),
            Self::V0(_) => Default::default(),
        }
    }
}
