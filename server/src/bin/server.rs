use anyhow::Context;
use pavex::server::{Server, ServerHandle, ShutdownMode};
use server::{
    configuration::Config,
    telemetry::{get_subscriber, init_telemetry},
};
use server_sdk::{build_application_state, run};
use std::time::Duration;
use tracing_log_error::log_error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("pavex_files".into(), "info".into(), std::io::stdout);
    init_telemetry(subscriber)?;

    // We isolate all the server setup and launch logic in a separate function
    // to have a single choke point where we make sure to log fatal errors
    // that will cause the application to exit.
    if let Err(e) = _main().await {
        log_error!(*e, "The application is exiting due to an error");
    }

    Ok(())
}

async fn _main() -> anyhow::Result<()> {
    // Load environment variables from a .env file, if it exists.
    let _ = dotenvy::dotenv();

    let config = Config::load(None)?;
    let application_state = build_application_state(config.app).await;
    let tcp_listener = config
        .server
        .listener()
        .await
        .context("Failed to bind the server TCP listener")?;
    let address = tcp_listener
        .local_addr()
        .context("The server TCP listener doesn't have a local socket address")?;
    let server_builder = Server::new().listen(tcp_listener);

    tracing::info!("Starting to listen for incoming requests at {}", address);
    let server_handle = run(server_builder, application_state);
    graceful_shutdown(
        server_handle.clone(),
        config.server.graceful_shutdown_timeout,
    )
    .await;
    server_handle.await;
    Ok(())
}

async fn graceful_shutdown(server_handle: ServerHandle, timeout: Duration) {
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for the Ctrl+C signal");
        server_handle
            .shutdown(ShutdownMode::Graceful { timeout })
            .await;
    });
}
