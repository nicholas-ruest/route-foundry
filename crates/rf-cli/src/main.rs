use clap::{Parser, Subcommand};
use rf_errors::Result;
use rf_orchestrator::FoundryService;
use rf_repository::Repository;
use std::{net::SocketAddr, path::PathBuf};
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(long, env = "RF_DATA_DIR", default_value = ".route-foundry-data")]
    data_dir: PathBuf,
}
#[derive(Subcommand)]
enum Command {
    Serve {
        #[arg(long, default_value = "127.0.0.1:8080")]
        addr: SocketAddr,
    },
    Seed,
    Benchmark,
}
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let c = Cli::parse();
    let service = FoundryService::new(Repository::open(&c.data_dir)?)?;
    match c.command {
        Command::Serve { addr } => {
            let app = rf_api::router(rf_api::AppState {
                service,
                auth_mode: "dev-session".into(),
            });
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .map_err(|e| rf_errors::FoundryError::Dependency(e.to_string()))?;
            tracing::info!(%addr,"route_foundry_listening");
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = tokio::signal::ctrl_c().await;
                })
                .await
                .map_err(|e| rf_errors::FoundryError::Dependency(e.to_string()))?
        }
        Command::Seed => {
            let r = service.route(rf_seed::seed_request("seed-route"), "seed-correlation")?;
            println!(
                "{}",
                serde_json::to_string_pretty(&r)
                    .map_err(|e| rf_errors::FoundryError::Invalid(e.to_string()))?
            )
        }
        Command::Benchmark => println!(
            "{}",
            serde_json::to_string_pretty(&rf_evaluation::run_benchmark())
                .map_err(|e| rf_errors::FoundryError::Invalid(e.to_string()))?
        ),
    }
    Ok(())
}
