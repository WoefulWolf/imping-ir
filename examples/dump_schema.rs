//! Prints the IR JSON Schema to stdout.
//!
//! Run with: `cargo run --example dump_schema --features schema`
//! Redirect to a file to capture it, e.g. `… > imping-ir.schema.json`.

#[cfg(feature = "schema")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = imping_ir::json_schema();
    println!("{}", serde_json::to_string_pretty(&schema)?);
    Ok(())
}

#[cfg(not(feature = "schema"))]
fn main() {
    eprintln!("the JSON Schema requires the `schema` feature: re-run with --features schema");
    std::process::exit(1);
}
