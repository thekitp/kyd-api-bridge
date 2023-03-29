use std::net::SocketAddr;

use axum::{response::Result, routing::get, Json, Router};
use kyd_api_bridge::{Analytics, KYDSensorsData};
use tracing::debug;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("VAM_LOG"))
        .init();

    let webapi = Router::new()
        .route("/", get(root))
        .route("/analytics", get(get_analytics));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    debug!("listenning on {addr}");

    axum::Server::bind(&addr)
        .serve(webapi.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_analytics() -> Result<Json<Analytics>> {
    let mut result = Analytics::default();

    let kyd_sensor = KYDSensorsData::fetch().await.unwrap();

    result.with_sensor_data(kyd_sensor);

    Ok(axum::Json::from(result))
}
