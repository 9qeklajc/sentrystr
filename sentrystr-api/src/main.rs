use clap::Parser;
use sentrystr_api::create_app;
use std::net::SocketAddr;

#[derive(Parser)]
#[command(name = "sentrystr-api")]
#[command(about = "REST API server for SentryStr events")]
struct Cli {
    #[arg(short, long, default_value = "3000")]
    port: u16,

    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let app = create_app();

    let addr = SocketAddr::new(cli.host.parse()?, cli.port);

    println!("SentryStr API server starting on {}", addr);
    println!("Health endpoint: http://{}/health", addr);
    println!("Events endpoint: http://{}/events", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
