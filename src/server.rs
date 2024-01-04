use crate::metrics::collect_data;
use crate::metrics::HostMetrics;
use crate::si::SelectionInput;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Json;
use axum::Router;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Span;
#[tracing::instrument]
fn create_app() -> Router {
    let state = Arc::new(Mutex::new(HostMetrics::new()));

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/v1/metrics", get(host_metrics))
        .layer(TraceLayer::new_for_http().on_request(
            |request: &axum::http::Request<_>, _span: &Span| {
                tracing::debug!("request: {:?}", request);
            },
        ))
        .with_state(state);

    app
}

#[tracing::instrument]
async fn create_listener(bind_addr: &str) -> Result<TcpListener, tokio::io::Error> {
    tracing::info!("Creating listener on {}", bind_addr);
    TcpListener::bind(bind_addr).await
}

#[tracing::instrument]
pub async fn serve(bind_addr: &str) -> Result<(), tokio::io::Error> {
    let listener = create_listener(bind_addr).await?;
    let app = create_app();

    tracing::info!("Listening on {}", bind_addr);
    axum::serve(listener, app).await
}

async fn ping() -> &'static str {
    "Pong!"
}

#[tracing::instrument(skip(state))]
async fn host_metrics(
    State(state): State<Arc<Mutex<HostMetrics>>>,
) -> (StatusCode, Json<SelectionInput>) {
    let si = collect_data(state);

    (StatusCode::OK, Json(si))
}
