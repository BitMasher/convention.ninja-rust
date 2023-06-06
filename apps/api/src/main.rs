#![forbid(unsafe_code)]


use std::sync::Arc;
use convention_ninja_rust::{Context, run};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let context = Arc::new(Context{});
    run(context).await;
    Ok(())
}