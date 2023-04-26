use super::actions::ChartExtActions;
use super::features::ChartExtFeatures;
use super::ui_schema::UiSchema;
use crate::metadata::ChartMetadata;
use crate::resource_types::ChartExtResourceTypes;
use serde::{de::DeserializeOwned, Serialize};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tokio::fs::{self, read_to_string};
use tokio::try_join;

#[derive(Debug)]
pub struct ChartExt {
    pub metadata: Option<ChartMetadata>,
    pub ui_schema: Option<UiSchema>,
    pub actions: Option<ChartExtActions>,
    pub features: Option<ChartExtFeatures>,
    pub resource_types: Option<ChartExtResourceTypes>,
    pub error: Option<String>,
}

impl ChartExt {
    pub async fn from_path(path: &Path) -> Result<Self, std::io::Error> {
        match read_chart(path).await {
            Ok((metadata, ui_schema, actions, features, resource_types)) => Ok(Self {
                metadata: Some(metadata),
                ui_schema,
                actions,
                features,
                resource_types,
                error: None,
            }),
            Err(ChartExtError::IoError(err)) => Err(err),
            Err(error) => Ok(Self {
                metadata: None,
                ui_schema: None,
                actions: None,
                features: None,
                resource_types: None,
                error: Some(error.to_string()),
            }),
        }
    }

    pub fn new_with_error(error: String) -> Self {
        Self {
            metadata: None,
            ui_schema: None,
            actions: None,
            features: None,
            resource_types: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum ChartExtError {
    #[error("std::io::Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Could not find Chart.yaml in {0}")]
    NoChartYaml(PathBuf),
    #[error("Error while parsing {0}: {1}")]
    ParseError(String, String),
}

async fn platz_dir_exists(path: &Path) -> Result<bool, std::io::Error> {
    match fs::metadata(path.join("platz")).await {
        Ok(metadata) if metadata.is_dir() => Ok(true),
        Ok(_) => Ok(false),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
        Err(err) => Err(err),
    }
}

async fn read_chart(
    path: &Path,
) -> Result<
    (
        ChartMetadata,
        Option<UiSchema>,
        Option<ChartExtActions>,
        Option<ChartExtFeatures>,
        Option<ChartExtResourceTypes>,
    ),
    ChartExtError,
> {
    let metadata = try_read_chart_metadata(path).await?;
    let (ui_schema, actions, features, resource_types) = if platz_dir_exists(path).await? {
        try_read_chart_extensions(
            path,
            Some("platz/values-ui.yaml"),
            Some("platz/actions.yaml"),
            Some("platz/features.yaml"),
            Some("platz/resources.yaml"),
        )
        .await?
    } else {
        try_read_chart_extensions(
            path,
            Some("values.ui.json"),
            Some("actions.schema.json"),
            Some("features.json"),
            None,
        )
        .await?
    };
    Ok((metadata, ui_schema, actions, features, resource_types))
}

async fn try_read_chart_metadata(chart_path: &Path) -> Result<ChartMetadata, ChartExtError> {
    read_spec_file(chart_path, Some("Chart.yaml"))
        .await?
        .ok_or_else(|| ChartExtError::NoChartYaml(chart_path.into()))
}

async fn try_read_chart_extensions(
    chart_path: &Path,
    ui_schema_filename: Option<&str>,
    actions_filename: Option<&str>,
    features_filename: Option<&str>,
    resource_types_filename: Option<&str>,
) -> Result<
    (
        Option<UiSchema>,
        Option<ChartExtActions>,
        Option<ChartExtFeatures>,
        Option<ChartExtResourceTypes>,
    ),
    ChartExtError,
> {
    Ok(try_join!(
        read_spec_file(chart_path, ui_schema_filename),
        read_spec_file(chart_path, actions_filename),
        read_spec_file(chart_path, features_filename),
        read_spec_file(chart_path, resource_types_filename),
    )?)
}

async fn read_spec_file<T>(path: &Path, filename: Option<&str>) -> Result<Option<T>, ChartExtError>
where
    T: Serialize + DeserializeOwned,
{
    let Some(filename) = filename else {
        return Ok(None)
    };

    let full_path = path.join(filename);

    let file_ext = full_path
        .extension()
        .and_then(|osstr| osstr.to_str())
        .map(ToString::to_string);

    let contents = match read_to_string(full_path).await {
        Ok(contents) => contents,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err.into()),
    };

    match file_ext.as_deref() {
        Some("yaml") | Some("yml") => {
            Ok(Some(serde_yaml::from_str(&contents).map_err(|err| {
                ChartExtError::ParseError(filename.to_owned(), err.to_string())
            })?))
        }
        Some("json") => Ok(Some(serde_json::from_str(&contents).map_err(|err| {
            ChartExtError::ParseError(filename.to_owned(), err.to_string())
        })?)),
        _ => Err(ChartExtError::ParseError(
            filename.to_owned(),
            "Unknown file extension".to_owned(),
        )),
    }
}
