use {{project-name}}::{App, Result};

#[tokio::main]
async fn main() -> Result<()> {
    App::new(60f64, 10f64)?.run().await
}
