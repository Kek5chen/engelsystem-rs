use engelsystem_rs_api::Result;
use engelsystem_rs_api::server::run_server;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> Result<()> {
    _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    run_server().await
}
