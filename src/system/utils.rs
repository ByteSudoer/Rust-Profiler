pub fn convert_megahz_to_gigahz(freq: f64) -> f64 {
    freq / 1000.0
}

#[allow(dead_code)]
pub fn convert_gigahz_to_megahz(freq: f64) -> f64 {
    freq * 1000.0
}

pub fn human_readable_size(size_bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut size = size_bytes as f64;
    let mut index = 0;

    while size >= 1024.0 && index < units.len() - 1 {
        size /= 1024.0;
        index += 1;
    }
    format!("{:.2} {}", size, units[index])
}

#[allow(dead_code)]
pub fn human_readable_to_bytes(size_str: &str) -> Option<u64> {
    let units: Vec<&str> = vec!["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut size_splited = size_str.split_whitespace();
    if let Some(size_val) = size_splited.next() {
        if let Ok(size) = size_val.parse::<f64>() {
            if let Some(unit) = size_splited.next() {
                if let Some(unit_index) = units.iter().position(|&x| x == unit.to_uppercase()) {
                    let size_bytes = size * 1024f64.powi(unit_index as i32);
                    return Some(size_bytes as u64);
                }
            }
        }
    }
    tracing::error!("Error converting human readable units to bytes");
    None
}
