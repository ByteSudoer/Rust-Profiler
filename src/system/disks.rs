use crate::system::utils;
use serde::{Deserialize, Serialize};
use std::fmt;
use sysinfo::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
    device_name: String,
    disk_type: String,
    file_system: String,
    mount_point: String,
    is_removable: bool,
    total_space: String,
    available_space: String,
}

impl Disk {
    pub fn from_sysinfo_disk(value: &sysinfo::Disk) -> Self {
        Self {
            device_name: String::from(value.name().to_str().unwrap()),
            disk_type: match value.kind() {
                DiskKind::HDD => String::from("HDD"),
                DiskKind::SSD => String::from("SDD"),
                DiskKind::Unknown(_) => String::from("Unknown"),
            },
            file_system: String::from_utf8_lossy(value.file_system())
                .to_string()
                .to_ascii_uppercase(),
            mount_point: value.mount_point().to_string_lossy().to_string(),
            is_removable: value.is_removable(),
            total_space: utils::human_readable_size(value.total_space()),
            available_space: utils::human_readable_size(value.available_space()),
        }
    }
    pub fn get_free_size(&self) -> Option<(f64, f64)> {
        let total_space: f64 = utils::human_readable_to_bytes(&self.total_space).unwrap() as f64;
        let available_space: f64 =
            utils::human_readable_to_bytes(&self.available_space).unwrap() as f64;
        if total_space == 0.0 {
            return None;
        }
        let available_space_percentage = (available_space / total_space) * 100.0;
        let used_space_percentage = ((total_space - available_space) / total_space) * 100.0;
        Some((available_space_percentage, used_space_percentage))
    }
}
impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Disk Name : {}", self.device_name)?;
        writeln!(f, "Disk Type : {}", self.disk_type)?;
        writeln!(f, "FileSystem : {}", self.file_system)?;
        writeln!(f, "Mount Point : {}", self.mount_point)?;
        writeln!(f, "Removable : {}", self.is_removable)?;
        writeln!(f, "Total Space : {}", self.total_space)?;
        writeln!(
            f,
            "Available Space : {} ({:.2} %)",
            self.available_space,
            self.get_free_size().unwrap().0,
        )?;
        writeln!(
            f,
            "Used Space : {} ({:.2} %)",
            utils::human_readable_size(
                utils::human_readable_to_bytes(&self.total_space).unwrap()
                    - utils::human_readable_to_bytes(&self.available_space).unwrap()
            ),
            self.get_free_size().unwrap().1
        )?;
        Ok(())
    }
}

impl Default for Disk {
    fn default() -> Self {
        Self {
            device_name: "device_name".to_string(),
            disk_type: "type".to_string(),
            file_system: "file_system".to_string(),
            mount_point: "mount".to_string(),
            is_removable: false,
            total_space: "0 MB".to_string(),
            available_space: "0 MB".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disks {
    disks: Vec<Disk>,
}

impl Disks {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_disks();
        let disks_vec: Vec<Disk> = sys.disks().iter().map(Disk::from_sysinfo_disk).collect();
        Self { disks: disks_vec }
    }
}
impl Default for Disks {
    fn default() -> Self {
        let vec_disks = vec![Disk::default(), Disk::default()];
        Self { disks: vec_disks }
    }
}
impl fmt::Display for Disks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for disk in &self.disks {
            writeln!(f, "Disk : {}", disk)?;
        }
        Ok(())
    }
}
