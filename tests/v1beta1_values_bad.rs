mod fake_db;
mod utils;

use anyhow::Result;
use utils::load_chart;

#[tokio::test]
async fn test() -> Result<()> {
    let chart_ext = load_chart("v1beta1/chart2").await?;
    println!("{:?}", chart_ext);
    assert!(chart_ext.ui_schema.is_none());
    Ok(())
}
