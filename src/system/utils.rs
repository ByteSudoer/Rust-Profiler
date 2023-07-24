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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes() {
        assert_eq!(human_readable_size(100), "100.00 B");
        assert_eq!(human_readable_size(999), "999.00 B");
        assert_eq!(human_readable_to_bytes("100 B"), Some(100));
        assert_eq!(human_readable_to_bytes("999 B"), Some(999));
    }

    #[test]
    fn test_kilobytes() {
        assert_eq!(human_readable_size(2048), "2.00 KB");
        assert_eq!(human_readable_size(3072), "3.00 KB");
        assert_eq!(human_readable_to_bytes("2 KB"), Some(2 * 1024));
        assert_eq!(human_readable_to_bytes("3 KB"), Some(3 * 1024));
    }

    #[test]
    fn test_megabytes() {
        assert_eq!(human_readable_size(2097152), "2.00 MB");
        assert_eq!(human_readable_size(3145728), "3.00 MB");
        assert_eq!(human_readable_to_bytes("2 MB"), Some(2 * 1024 * 1024));
        assert_eq!(human_readable_to_bytes("3 MB"), Some(3 * 1024 * 1024));
    }

    #[test]
    fn test_gigabytes() {
        assert_eq!(human_readable_size(2147483648), "2.00 GB");
        assert_eq!(human_readable_size(3221225472), "3.00 GB");
        assert_eq!(
            human_readable_to_bytes("2 GB"),
            Some(2 * 1024 * 1024 * 1024)
        );
        assert_eq!(
            human_readable_to_bytes("3 GB"),
            Some(3 * 1024 * 1024 * 1024)
        );
    }

    #[test]
    fn test_terabytes() {
        assert_eq!(human_readable_size(2199023255552), "2.00 TB");
        assert_eq!(human_readable_size(3298534883328), "3.00 TB");
        assert_eq!(
            human_readable_to_bytes("2 TB"),
            Some(2 * 1024 * 1024 * 1024 * 1024)
        );
        assert_eq!(
            human_readable_to_bytes("3 TB"),
            Some(3 * 1024 * 1024 * 1024 * 1024)
        );
    }

    #[test]
    fn test_large_size() {
        assert_eq!(human_readable_size(1125899906842624), "1.00 PB");
        assert_eq!(human_readable_size(11258999068426240), "10.00 PB");
    }

    #[test]
    fn test_max_size() {
        assert_eq!(human_readable_size(u64::MAX), "16.00 EB");
    }
    #[test]
    fn test_invalid_formats() {
        assert_eq!(human_readable_to_bytes("1KB"), None); // Missing space between value and unit
        assert_eq!(human_readable_to_bytes("1.5TB"), None); // Missing space between value and unit
        assert_eq!(human_readable_to_bytes("ABC MB"), None); // Invalid value
        assert_eq!(human_readable_to_bytes("100 PBX"), None); // Unit not recognized
    }
}
