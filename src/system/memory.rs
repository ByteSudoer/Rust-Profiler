use crate::system::utils;
use serde::{Deserialize, Serialize};
use std::fmt;
use sysinfo::*;
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Memory {
    ram_size: String,
    swap_size: String,
}

impl Memory {
    pub fn new() -> Self {
        let sys = sysinfo::System::new_all();
        Self {
            ram_size: utils::human_readable_size(sys.total_memory()),
            swap_size: utils::human_readable_size(sys.total_swap()),
        }
    }
    pub fn get_used_percent(&self) -> f64 {
        let mut sys = sysinfo::System::new_all();
        let total_memory = sys.total_memory();
        sys.refresh_all();
        (sys.used_memory() as f64 / total_memory as f64) * 100.0
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Total RAM size : {}", self.ram_size)?;
        writeln!(f, "Total SWAP size : {}", self.swap_size)?;
        Ok(())
    }
}
