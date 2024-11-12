use {{project-name}}::{logger_init, App, Result};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    logger_init();

    info!("Starting application");

    App::new(60f64, 10f64).await?.run().await
}
