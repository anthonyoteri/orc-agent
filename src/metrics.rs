use crate::si::SelectionInput;
use std::sync::Arc;
use std::sync::Mutex;
use sysinfo::{Disks, Networks, System};

#[derive(Debug)]
pub struct HostMetrics {
    system: System,
    networks: Networks,
    disks: Disks,
}

impl HostMetrics {
    #[tracing::instrument]
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

    pub fn system(&self) -> &System {
        &self.system
    }

    pub fn networks(&self) -> &Networks {
        &self.networks
    }

    pub fn disks(&self) -> &Disks {
        &self.disks
    }
}

#[tracing::instrument(skip(state))]
fn refresh_all(state: &Mutex<HostMetrics>) {
    tracing::debug!("Refreshing hardware metrics");
    let mut state = state.lock().unwrap();
    state.system.refresh_all();
    state.networks.refresh();
    state.disks.refresh();
}

#[tracing::instrument]
pub fn init() -> HostMetrics {
    sysinfo::set_open_files_limit(0);
    HostMetrics::new()
}

#[tracing::instrument(skip(state))]
pub fn collect_data(state: Arc<Mutex<HostMetrics>>) -> SelectionInput {
    refresh_all(&state);
    SelectionInput::from(&*state.lock().unwrap())
}
