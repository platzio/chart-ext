use crate::versions::HelmChartV2;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[serde(from = "HelmChart")]
pub struct ChartMetadata {
    pub version: String,
    pub git_commit: Option<String>,
    pub git_branch: Option<String>,
    pub git_repo: Option<Url>,
    pub git_provider: Option<String>,
}

impl From<HelmChart> for ChartMetadata {
    fn from(chart: HelmChart) -> Self {
        let annotations = chart.annotations.as_ref();
        Self {
            version: chart.version,
            git_commit: annotations.and_then(|a| a.git_commit.to_owned()),
            git_branch: annotations.and_then(|a| a.git_branch.to_owned()),
            git_repo: annotations.and_then(|a| a.git_repo.to_owned()),
            git_provider: annotations.and_then(|a| a.git_provider.to_owned()),
        }
    }
}

/// The Chart.yaml file
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HelmChart {
    #[allow(dead_code)]
    api_version: HelmChartV2,
    version: String,
    annotations: Option<HelmChartAnnotations>,
}

#[derive(Deserialize)]
struct HelmChartAnnotations {
    #[serde(rename = "platz.io/git/commit")]
    git_commit: Option<String>,
    #[serde(rename = "platz.io/git/branch")]
    git_branch: Option<String>,
    #[serde(rename = "platz.io/git/repo")]
    git_repo: Option<Url>,
    #[serde(rename = "platz.io/git/provider")]
    git_provider: Option<String>,
}
