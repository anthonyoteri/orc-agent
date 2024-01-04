use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Parser)]
struct Args {
    /// The address on which to bind the HTTP server
    #[arg(short, long, default_value = "0.0.0.0:3000")]
    bind_address: String,

    /// An optional pre-shared API key which must be supplied in the
    /// `X-Api-Key` header for all incoming requests.
    #[arg(short = 'k', long, required = false)]
    pre_shared_key: Option<String>,
}

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "orc_agent=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    orc_agent::server::serve(&args.bind_address, args.pre_shared_key)
        .await
        .unwrap();
}
