mod fake_db;
mod utils;

use anyhow::Result;
use fake_db::TestDb;
use platz_chart_ext::UiSchema;
use serde_json::json;
use utils::load_chart;
use uuid::Uuid;

#[tokio::test]
async fn test_single_collection() -> Result<()> {
    let chart_ext = load_chart("v0/chart1").await?;
    let ui_schema = chart_ext.ui_schema.expect("No ui_schema");
    assert!(matches!(ui_schema, UiSchema::V0(_)));
    let inputs = json!({
        "a": "3",
    });
    let values: serde_json::Value = ui_schema
        .get_values::<TestDb>(Uuid::new_v4(), &inputs)
        .await?
        .into();
    let expected = json!({
        "config": {
            "a": {
                "id": "3",
                "value": "a3",
            }
        }
    });
    assert_eq!(values, expected);
    Ok(())
}

#[tokio::test]
async fn test_array_of_collection() -> Result<()> {
    let chart_ext = load_chart("v0/chart2").await?;
    let ui_schema = chart_ext.ui_schema.expect("No ui_schema");
    assert!(matches!(ui_schema, UiSchema::V0(_)));
    let inputs = json!({
        "a": ["3", "4"],
    });
    let values: serde_json::Value = ui_schema
        .get_values::<TestDb>(Uuid::new_v4(), &inputs)
        .await?
        .into();
    let expected = json!({
        "config": {
            "a": {
                "id": ["3", "4"],
                "value": ["a3", "a4"],
            }
        }
    });
    assert_eq!(values, expected);
    Ok(())
}

#[tokio::test]
async fn test_all_input_types() -> Result<()> {
    let chart_ext = load_chart("v0/chart3").await?;
    let ui_schema = chart_ext.ui_schema.expect("No ui_schema");
    assert!(matches!(ui_schema, UiSchema::V0(_)));
    Ok(())
}
