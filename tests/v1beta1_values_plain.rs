mod fake_db;
mod utils;

use anyhow::Result;
use fake_db::TestDb;
use platz_chart_ext::UiSchema;
use serde_json::json;
use utils::load_chart;
use uuid::Uuid;

#[tokio::test]
async fn test() -> Result<()> {
    let chart_ext = load_chart("v1beta1/chart1").await?;
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

    chart_ext.features.expect("No features");

    let resource_types = chart_ext.resource_types.expect("No resource types");
    assert_eq!(resource_types.0.len(), 1);

    Ok(())
}
