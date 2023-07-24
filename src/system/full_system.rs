use crate::system::cpu::Cpu;
use crate::system::disks::Disks;
use raw_cpuid::CpuId;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FullSystem {
    cpu: Cpu,
    disks: Disks,
}

impl FullSystem {
    pub fn new() -> Self {
        let cpuid = CpuId::new();
        Self {
            cpu: cpuid.into(),
            disks: Disks::new(),
        }
    }
}

impl fmt::Display for FullSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cpu : {}", self.cpu)?;
        writeln!(f, "Disks : {}", self.disks)?;
        Ok(())
    }
}
