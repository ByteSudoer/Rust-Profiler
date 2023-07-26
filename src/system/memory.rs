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
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Total RAM size : {}", self.ram_size)?;
        writeln!(f, "Total SWAP size : {}", self.swap_size)?;
        Ok(())
    }
}
