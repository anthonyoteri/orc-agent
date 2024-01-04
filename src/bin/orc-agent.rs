use orc_agent::metrics::collect_data;
use orc_agent::metrics::init;

use std::sync::Arc;
use std::sync::Mutex;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {}

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let state = Arc::new(Mutex::new(init()));
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3));
        let si = collect_data(state.clone());
        println!("{}", serde_json::to_string_pretty(&si).unwrap());
    }
}
