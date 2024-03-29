mod fake_db;
mod utils;

use std::collections::BTreeMap;

use anyhow::Result;
use fake_db::TestDb;
use platz_chart_ext::{
    ChartExtDeploymentDisplay, ChartExtDeploymentDisplayIcon, ChartExtDeploymentDisplayName,
    ChartExtDeploymentDisplayNameInputField, ChartExtIngressHostnameFormat, UiSchema,
};
use serde_json::json;
use url::Url;
use utils::load_chart;
use uuid::Uuid;

#[tokio::test]
async fn test1() -> Result<()> {
    let chart_ext = load_chart("v1beta2/chart1").await?;

    let metadata = chart_ext.metadata.expect("Chart has no metadata");
    assert_eq!(metadata.version, "1.0.0");
    assert!(metadata.git_commit.is_some());
    assert_eq!(metadata.git_branch, Some("main".to_owned()));
    assert_eq!(
        metadata.git_repo,
        Some(Url::parse("https://github.com/platzio/chart-ext").unwrap())
    );
    assert_eq!(metadata.git_provider, Some("github".to_owned()));

    let ui_schema = chart_ext.ui_schema.expect("No ui_schema");
    assert!(matches!(ui_schema, UiSchema::V1Beta1(_)));
    let inputs = json!({
        "required_bool": true,
        "required_num": 3,
        "required_text": "blah",
        "ignored_field": 5,
        "array_of_text": ["value"]
    });
    let values: serde_json::Value = ui_schema
        .get_values::<TestDb>(Uuid::new_v4(), &inputs)
        .await?
        .into();
    let expected = json!({
        "config": {
            "required_bool": true,
            "required_num": 3,
            "required_text": "blah",
            "array_of_text": ["value"]
        }
    });
    assert_eq!(values, expected);

    chart_ext.actions.expect("No actions");

    let features = chart_ext.features.expect("No features");
    assert!(features.ingress().enabled);
    assert!(matches!(
        features.ingress().hostname_format,
        ChartExtIngressHostnameFormat::KindAndName
    ));
    assert_eq!(
        features.display(),
        ChartExtDeploymentDisplay {
            name: None,
            icon: Some(ChartExtDeploymentDisplayIcon {
                font_awesome: "rocket".to_owned(),
            })
        }
    );

    let resource_types = chart_ext.resource_types.expect("No resource types");
    assert_eq!(resource_types.0.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test2() -> Result<()> {
    let chart_ext = load_chart("v1beta2/chart3").await?;
    let ui_schema = chart_ext.ui_schema.expect("No ui_schema");
    match ui_schema {
        UiSchema::V0(_) => panic!("Expected UiSchema::V1Beta1"),
        UiSchema::V1Beta1(schema) => {
            assert_eq!(schema.inner.outputs.secrets.0.len(), 2);
        }
    }

    chart_ext.actions.expect("No actions");

    let features = chart_ext.features.expect("No features");
    assert!(!features.ingress().enabled);
    assert!(matches!(
        features.ingress().hostname_format,
        ChartExtIngressHostnameFormat::Name
    ));
    assert_eq!(
        features.display(),
        ChartExtDeploymentDisplay {
            name: Some(ChartExtDeploymentDisplayName::InputField(
                ChartExtDeploymentDisplayNameInputField {
                    name: "alias".to_owned(),
                }
            )),
            icon: Some(ChartExtDeploymentDisplayIcon {
                font_awesome: "rocket".to_owned(),
            })
        }
    );

    let resource_types = chart_ext.resource_types.expect("No resource types");
    assert_eq!(resource_types.0.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test3() -> Result<()> {
    let chart_ext = load_chart("v1beta2/chart4").await?;

    let features = chart_ext.features.expect("No features");
    assert_eq!(
        features.display(),
        ChartExtDeploymentDisplay {
            name: Some(ChartExtDeploymentDisplayName::DeploymentName),
            icon: Some(ChartExtDeploymentDisplayIcon {
                font_awesome: "rocket".to_owned(),
            })
        }
    );

    Ok(())
}

#[tokio::test]
async fn test4() -> Result<()> {
    let chart_ext = load_chart("v1beta2/chart5").await?;

    let metadata = chart_ext.metadata.expect("Chart has no metadata");
    assert_eq!(metadata.version, "1.0.0");

    let ui_schema = chart_ext.ui_schema.expect("No ui_schema");
    assert!(matches!(ui_schema, UiSchema::V1Beta1(_)));

    // No showIf is met
    {
        let inputs = json!({
            "required_bool": false,
            "required_num": 3,
            "required_text": "blah",
            "conditional_bool": true,
            "conditional_text": "lolz",
        });

        let values: serde_json::Value = ui_schema
            .get_values::<TestDb>(Uuid::new_v4(), &inputs)
            .await?
            .into();
        let expected = json!({
            "config": {
                "required_num": 3,
                "required_text": "blah",
            }
        });
        assert_eq!(values, expected);
    }

    // First showIf is met
    {
        let inputs = json!({
            "required_bool": true,
            "required_num": 3,
            "required_text": "blah",
            "conditional_bool": true,
            "conditional_text": "lolz",
        });

        let values: serde_json::Value = ui_schema
            .get_values::<TestDb>(Uuid::new_v4(), &inputs)
            .await?
            .into();
        let expected = json!({
            "config": {
                "required_num": 3,
                "required_text": "blah",
                "conditional_bool": true,
            }
        });
        assert_eq!(values, expected);
    }

    // Second showIf is met
    {
        let inputs = json!({
            "required_bool": false,
            "required_num": 500,
            "required_text": "aaaah",
            "conditional_text": "lolz",
        });

        let values: serde_json::Value = ui_schema
            .get_values::<TestDb>(Uuid::new_v4(), &inputs)
            .await?
            .into();
        let expected = json!({
            "config": {
                "required_num": 500,
                "required_text": "aaaah",
                "conditional_text": "lolz",
            }
        });
        assert_eq!(values, expected);
    }

    // Both showIfs are met
    {
        let inputs = json!({
            "required_bool": true,
            "required_num": 350,
            "required_text": "ah",
            "conditional_bool": false,
            "conditional_text": "hmpf",
        });

        let values: serde_json::Value = ui_schema
            .get_values::<TestDb>(Uuid::new_v4(), &inputs)
            .await?
            .into();
        let expected = json!({
            "config": {
                "required_num": 350,
                "required_text": "ah",
                "conditional_bool": false,
                "conditional_text": "hmpf",
            }
        });
        assert_eq!(values, expected);
    }

    Ok(())
}

#[tokio::test]
async fn test5() -> Result<()> {
    let chart_ext = load_chart("v1beta2/chart6").await?;

    let metadata = chart_ext.metadata.expect("Chart has no metadata");
    assert_eq!(metadata.version, "1.0.0");

    let ui_schema = chart_ext.ui_schema.expect("No ui_schema");
    assert!(matches!(ui_schema, UiSchema::V1Beta1(_)));

    // showIf is met, something selected and typed
    {
        let inputs = json!({
            "required_bool": true,
            "conditional_select": "123",
            "conditional_text": "condtext"
        });

        let values: serde_json::Value = ui_schema
            .get_values::<TestDb>(Uuid::new_v4(), &inputs)
            .await?
            .into();
        let expected = json!({
            "config": {
                "selected": {
                    "id": "123",
                    "a": "a123"
                }
            }
        });

        assert_eq!(values, expected);

        let secrets = ui_schema
            .get_secrets::<TestDb>(Uuid::new_v4(), &inputs)
            .await?;
        assert_eq!(secrets.len(), 1);
        let secret = &secrets[0];
        assert_eq!(secret.name, "secret-env");
        assert_eq!(
            secret.attrs,
            BTreeMap::from([
                ("SELECTED_SECRET".into(), "a123".into()),
                ("TYPED_SECRET".into(), "condtext".into())
            ])
        );
    }

    // showIf is not met
    {
        let inputs = json!({
            "required_bool": false,
        });

        let values: serde_json::Value = ui_schema
            .get_values::<TestDb>(Uuid::new_v4(), &inputs)
            .await?
            .into();
        let expected = json!({});
        assert_eq!(values, expected);

        let secrets = ui_schema
            .get_secrets::<TestDb>(Uuid::new_v4(), &inputs)
            .await?;
        assert!(secrets.is_empty());
    }

    // showIf is met, nothing selected, something typed
    {
        let inputs = json!({
            "required_bool": true,
            "conditional_text": "condtext"
        });

        ui_schema
            .get_values::<TestDb>(Uuid::new_v4(), &inputs)
            .await
            .expect_err("gotta fail");

        // Can't expect_err, since RenderedSecret is !Debug
        assert!(ui_schema
            .get_secrets::<TestDb>(Uuid::new_v4(), &inputs)
            .await
            .is_err());
    }

    // showIf is met, something selected, nothing typed
    {
        let inputs = json!({
            "required_bool": true,
            "conditional_select": "123",
        });

        let values: serde_json::Value = ui_schema
            .get_values::<TestDb>(Uuid::new_v4(), &inputs)
            .await?
            .into();
        let expected = json!({
            "config": {
                "selected": {
                    "id": "123",
                    "a": "a123"
                }
            }
        });

        assert_eq!(values, expected);

        // Can't expect_err, since RenderedSecret is !Debug
        assert!(ui_schema
            .get_secrets::<TestDb>(Uuid::new_v4(), &inputs)
            .await
            .is_err());
    }

    Ok(())
}
