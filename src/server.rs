use crate::metrics::collect_data;
use crate::metrics::HostMetrics;
use crate::si::SelectionInput;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Json;
use axum::Router;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Span;

#[derive(Debug)]
pub struct AppState {
    host_metrics: Arc<Mutex<HostMetrics>>,
    psk: Option<String>,
}

#[tracing::instrument]
fn create_app(psk: Option<String>) -> Router {
    let state = AppState {
        host_metrics: Arc::new(Mutex::new(HostMetrics::new())),
        psk,
    };

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/v1/metrics", get(host_metrics))
        .layer(TraceLayer::new_for_http().on_request(
            |request: &axum::http::Request<_>, _span: &Span| {
                tracing::debug!("request: {:?}", request);
            },
        ))
        .with_state(Arc::new(state));

    app
}

#[tracing::instrument]
async fn create_listener(bind_addr: &str) -> Result<TcpListener, tokio::io::Error> {
    tracing::info!("Creating listener on {}", bind_addr);
    TcpListener::bind(bind_addr).await
}

#[tracing::instrument]
pub async fn serve(bind_addr: &str, psk: Option<String>) -> Result<(), tokio::io::Error> {
    let listener = create_listener(bind_addr).await?;
    let app = create_app(psk);

    tracing::info!("Listening on {}", bind_addr);
    axum::serve(listener, app).await
}

async fn ping() -> &'static str {
    "Pong!"
}

#[tracing::instrument(skip(state))]
async fn host_metrics(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<SelectionInput>) {
    if let Some(psk) = &state.psk {
        if let Some(auth_header) = headers.get("X-Api-Key") {
            tracing::debug!("X-Api-Key: {:?}", auth_header);

            if auth_header != psk {
                tracing::debug!("Invalid API Key");
                return (StatusCode::UNAUTHORIZED, Json(SelectionInput::default()));
            }
        } else {
            tracing::debug!("Missing API Key");
            return (StatusCode::UNAUTHORIZED, Json(SelectionInput::default()));
        }
    }

    tracing::debug!("Got here");
    let si = collect_data(state.host_metrics.clone());

    (StatusCode::OK, Json(si))
}
