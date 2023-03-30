use std::net::SocketAddr;

use axum::{response::Result, routing::get, Json, Router};
use http::Method;
use kyd_api_bridge::{Analytics, KYDApi};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("VAM_LOG"))
        .init();

    let webapi = Router::new()
        .route("/analytics", get(get_analytics))
        .layer(CorsLayer::very_permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    debug!("listenning on {addr}");

    axum::Server::bind(&addr)
        .serve(webapi.into_make_service())
        .await?;

    Ok(())
}

async fn get_analytics() -> Result<Json<Vec<Analytics>>> {
    let mut analytics_results: Vec<Analytics> = Vec::new();
    let mut analytic_data = Analytics::default();

    let kydapi_pas_co2_sensor = KYDApi::fetch("TEST000012".into()).await.unwrap();

    for entry in kydapi_pas_co2_sensor.data {
        //
        info!("{:?}", entry);

        analytic_data.fill_device_data(entry);
    }

    let radar_human_entrance_counter_sensor = KYDApi::fetch("TEST000013".into()).await.unwrap();

    for entry in radar_human_entrance_counter_sensor.data {
        //
        info!("{:?}", entry);
        analytic_data.fill_device_data(entry);
    }

    analytics_results.push(analytic_data);

    Ok(axum::Json::from(analytics_results))
}
