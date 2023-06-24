use crate::actions::ChartExtActionEndpoint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChartExtFeaturesV0 {
    /// When true, values.yaml is injected with an `ingress` section
    /// that matches the structure generated by `helm create`. This
    /// generates the hostname and sets TLS correctly.
    #[serde(default)]
    pub standard_ingress: bool,

    /// Sets an HTTP endpoint that returns a platz_sdk::PlatzStatus
    /// and displayed as part of the deployment page.
    pub status: Option<ChartExtStatusFeature>,

    /// Allow deploying OnePerCluster or Many.
    #[serde(default)]
    pub cardinality: ChartExtCardinality,

    /// Should dependent deployments be reinstalled when this deployment
    /// config/values are updated. This doesn't apply to renames or
    /// moving between clusters which always reinstalls dependencies.
    #[serde(default = "yes")]
    pub reinstall_dependencies: bool,

    /// Paths to inject the node selector to. Node selector is always
    /// added at the values top level `nodeSelector`.
    #[serde(default)]
    pub node_selector_paths: Vec<Vec<String>>,

    /// Same for tolerations
    #[serde(default)]
    pub tolerations_paths: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChartExtStatusFeature {
    pub endpoint: ChartExtActionEndpoint,
    pub path: String,
    pub refresh_interval_secs: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ChartExtCardinality {
    #[default]
    Many,
    OnePerCluster,
}

fn yes() -> bool {
    true
}
