use engelsystem_rs::server::run_server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use engelsystem_rs::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    _ = dotenvy::dotenv();

    run_server().await
}
