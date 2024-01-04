use crate::metrics::State;
use std::collections::BTreeMap;
use std::time::SystemTime;
use sysinfo::System;

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct SelectionInput {
    pub system_name: String,
    pub hostname: String,
    pub kernel_version: String,
    pub os_version: String,
    pub uptime: u64,

    pub timestamp: u64,

    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,

    pub num_cpus: usize,
    pub load_average: Vec<f64>,

    pub cpu_usage: f32,

    pub network: BTreeMap<String, NetworkInfo>,
    pub volumes: BTreeMap<String, DiskInfo<'static>>,
}

#[derive(Debug, Default, Serialize)]
pub struct NetworkInfo {
    pub mac_address: String,
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub errors_in: u64,
    pub errors_out: u64,
}

#[derive(Debug, Default, Serialize)]
pub struct DiskInfo<'a> {
    pub device: String,
    pub kind: &'a str,
    pub total: u64,
    pub available: u64,
    pub used: u64,
}

impl From<&State> for SelectionInput {
    fn from(state: &State) -> Self {
        let load_average = System::load_average();

        Self {
            system_name: System::name().unwrap(),
            hostname: System::host_name().unwrap(),
            kernel_version: System::kernel_version().unwrap(),
            os_version: System::os_version().unwrap_or_default(),
            uptime: System::uptime(),

            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),

            total_memory: state.system().total_memory(),
            used_memory: state.system().used_memory(),
            total_swap: state.system().total_swap(),
            used_swap: state.system().used_swap(),

            num_cpus: state.system().cpus().len(),

            cpu_usage: state.system().global_cpu_info().cpu_usage(),
            load_average: vec![load_average.one, load_average.five, load_average.fifteen],

            network: state
                .networks()
                .into_iter()
                .map(|(name, network)| {
                    (
                        name.to_string(),
                        NetworkInfo {
                            mac_address: network.mac_address().to_string(),
                            bytes_in: network.received(),
                            bytes_out: network.transmitted(),
                            errors_in: network.errors_on_received(),
                            errors_out: network.errors_on_transmitted(),
                        },
                    )
                })
                .collect(),

            volumes: state
                .disks()
                .into_iter()
                .map(|disk| {
                    (
                        disk.mount_point().to_string_lossy().to_string(),
                        DiskInfo {
                            device: disk.name().to_string_lossy().to_string(),
                            kind: match disk.kind() {
                                sysinfo::DiskKind::HDD => "HDD",
                                sysinfo::DiskKind::SSD => "SSD",
                                sysinfo::DiskKind::Unknown(_) => "Unknown",
                            },
                            total: disk.total_space(),
                            available: disk.available_space(),
                            used: disk.total_space() - disk.available_space(),
                        },
                    )
                })
                .collect(),
        }
    }
}
