use wasm_bindgen::prelude::*;

use std::sync::Arc;

use datafusion::arrow::array::{Int32Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;

use datafusion::datasource::MemTable;
// use datafusion::error::Result;
use datafusion::prelude::*;

use arrow::util::pretty;

use arrow::error::Result as ArrowResult;
use std::fmt::Display;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    console_error_panic_hook::set_once();
}

#[no_mangle]
#[wasm_bindgen]
pub fn start_running() -> i32 {
    set_panic_hook();

    alert("start it");

    println!("Starting...");

    wasm_bindgen_futures::spawn_local(async {
        let result_string = run().await.unwrap();

        alert("got result");

        alert(format!("{}", result_string).as_str());
    });

    0
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// NOTE: tokio/time uses std::time which is not implemented in the browser
// instead of using current_thread, which we don't actually need, we can
// call this function using wasm-bindgen-futures
// #[tokio::main(flavor = "current_thread")]
#[no_mangle]
async fn run() -> ArrowResult<impl Display> {
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
    // df.show().await?;

    let result_string = pretty::pretty_format_batches(&df.collect().await.unwrap()).unwrap();

    Ok(result_string)
}
