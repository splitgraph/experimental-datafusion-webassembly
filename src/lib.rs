
use std::sync::Arc;

use datafusion::arrow::array::{Int32Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;

use datafusion::datasource::MemTable;
use datafusion::error::Result;
use datafusion::prelude::*;

#[no_mangle]
pub fn _start() -> i32 {
    let result = run();
    match result {
        Ok(_) => 0,
        Err(err) => {
            println!("Error: {}", err);
            1
        },
    }
}

#[tokio::main(flavor = "current_thread")]
async fn run() -> Result<()>{
    // define a schema.
    let schema = Arc::new(Schema::new(vec![
        Field::new("a", DataType::Utf8, false),
        Field::new("b", DataType::Int32, false),
    ]));

    // define data.
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(vec!["a", "b", "c", "d"])),
            Arc::new(Int32Array::from(vec![1, 10, 10, 100])),
        ],
    )?;

    // Register table
    let ctx = SessionContext::new();
    let provider = MemTable::try_new(schema, vec![vec![batch]])?;
    ctx.register_table("t", Arc::new(provider))?;

    // Execute query
    let df = ctx.sql("SELECT a, b FROM t WHERE b = 10").await?;

    // Show results
    // let results = df.collect().await?;
    df.show().await?;

    Ok(())
}
