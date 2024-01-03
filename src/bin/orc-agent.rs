use orc_agent::collect_data;
use orc_agent::init;

use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let state = Arc::new(Mutex::new(init()));
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3));
        let si = collect_data(state.clone());
        println!("{}", serde_json::to_string_pretty(&si).unwrap());
    }
}
