use crate::system::utils;
use raw_cpuid::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use sysinfo::*;
use tracing::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Frequency {
    base: f64,
    max: f64,
}

impl<R: CpuIdReader> From<CpuId<R>> for Frequency {
    fn from(value: CpuId<R>) -> Self {
        Self {
            base: value
                .get_processor_frequency_info()
                .unwrap()
                .processor_base_frequency() as f64,
            max: value
                .get_processor_frequency_info()
                .unwrap()
                .processor_max_frequency() as f64,
        }
    }
}
impl Default for Frequency {
    fn default() -> Self {
        Self {
            base: 0.0,
            max: 0.0,
        }
    }
}

pub fn get_cpu_core_count() -> Option<usize> {
    let sys = sysinfo::System::new();
    match sys.physical_core_count() {
        Some(count) => Some(count),
        None => {
            error!("Could not get the number of physical cores");
            None
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Cpu {
    vendor: String,
    model: String,
    num_cores: Option<usize>,
    frequency: Frequency,
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Frequency : ")?;
        writeln!(
            f,
            "\tMax : {}GHz ({}MHz)",
            utils::convert_megahz_to_gigahz(self.max),
            self.max
        )?;
        writeln!(
            f,
            "\tBase : {}GHz ({}MHz)",
            utils::convert_megahz_to_gigahz(self.base),
            self.base
        )?;
        Ok(())
    }
}

impl<R: CpuIdReader> From<CpuId<R>> for Cpu {
    fn from(cpu_source: CpuId<R>) -> Self {
        Self {
            vendor: cpu_source
                .get_vendor_info()
                .as_ref()
                .map_or_else(|| "unknown", |vf| vf.as_str())
                .to_string(),
            model: cpu_source
                .get_processor_brand_string()
                .as_ref()
                .map_or_else(|| "n/a", |pbs| pbs.as_str())
                .to_string(),
            num_cores: get_cpu_core_count(),
            frequency: cpu_source.into(),
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        info!("Display Structure CPU");
        writeln!(f, "Cpu :")?;
        writeln!(f, "\tVendor : {}", self.vendor)?;
        writeln!(f, "\tModel : {}", self.model)?;
        writeln!(
            f,
            "\tNumber of Physical Cores : {}",
            self.num_cores.unwrap()
        )?;
        writeln!(f, "\t{}", self.frequency)?;
        Ok(())
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            vendor: "vendor".to_string(),
            model: "brand".to_string(),
            num_cores: Some(2),
            frequency: Frequency::default(),
        }
    }
}
