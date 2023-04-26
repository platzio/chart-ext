mod fake_db;
mod utils;

pub use anyhow::Result;
pub use fake_db::TestDb;
pub use platz_chart_ext::*;
pub use serde_json::json;
use utils::load_chart;
use uuid::Uuid;

#[tokio::test]
async fn test() -> Result<()> {
    let chart_ext = load_chart("v0/chart4").await?;
    let ui_schema = chart_ext.ui_schema.expect("No ui_schema");

    let inputs1 = json!({
        "required_enum": "value2",
    });
    let values1: serde_json::Value = ui_schema
        .get_values::<TestDb>(Uuid::new_v4(), &inputs1)
        .await?
        .into();
    let expected1 = json!({
        "config": {
            "required_enum": "value2",
        }
    });
    assert_eq!(values1, expected1);

    let inputs2 = json!({
        "required_enum": "value3",
    });
    let _missing = "required_dependent_num".to_owned();
    assert!(std::matches!(
        ui_schema
            .get_values::<TestDb>(Uuid::new_v4(), &inputs2)
            .await,
        Err(UiSchemaInputError::MissingInputValue(_missing))
    ));

    let inputs3 = json!({
        "required_enum": "value3",
        "required_dependent_num": 5,
    });
    let values3: serde_json::Value = ui_schema
        .get_values::<TestDb>(Uuid::new_v4(), &inputs3)
        .await?
        .into();
    let expected3 = json!({
        "config": {
            "required_enum": "value3",
            "required_dependent_num": 5,
        }
    });
    assert_eq!(values3, expected3);

    Ok(())
}
