mod si;

use si::SelectionInput;
use std::sync::Arc;
use std::sync::Mutex;
use sysinfo::{Disks, Networks, System};

#[derive(Debug)]
pub struct State {
    system: System,
    networks: Networks,
    disks: Disks,
}

impl State {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let networks = Networks::new_with_refreshed_list();
        let disks = Disks::new_with_refreshed_list();
        Self {
            system,
            networks,
            disks,
        }
    }
}

fn refresh_all(state: &Mutex<State>) {
    let mut state = state.lock().unwrap();
    state.system.refresh_all();
    state.networks.refresh();
    state.disks.refresh();
}

pub fn init() -> State {
    sysinfo::set_open_files_limit(0);
    State::new()
}

pub fn collect_data(state: Arc<Mutex<State>>) -> SelectionInput {
    refresh_all(&state);
    SelectionInput::from(&*state.lock().unwrap())
}
