pub fn convert_megahz_to_gigahz(freq: f64) -> f64 {
    freq / 1000.0
}

pub fn convert_gigahz_to_megahz(freq: f64) -> f64 {
    freq * 1000.0
}

#[cfg(test)]
mod tests {}
