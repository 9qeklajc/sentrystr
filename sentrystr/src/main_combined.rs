use sentrystr::combined_example::run_combined_example;

#[tokio::main]
async fn main() -> sentrystr::Result<()> {
    println!("🚀 Running Rust Combined Example");

    match run_combined_example().await {
        Ok(()) => {
            println!("✅ Combined example completed successfully!");
        }
        Err(e) => {
            println!("❌ Combined example failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
