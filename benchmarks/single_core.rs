use std::time::Instant;

#[allow(dead_code)]
pub fn max_single_core_usage() {
    tracing_subscriber::fmt::init();
    tracing::info!("Single core benchmark started");
    let start_time = Instant::now();
    let num_iterations: u64 = 100_000_000;

    for _ in 0..num_iterations {
        let _ = (1000..).find(|&x| x * x >= 1_000_000);
    }
    let elapsed_time = start_time.elapsed().as_secs_f64();
    tracing::info!("Single core usage took {:.2} seconds", elapsed_time);
}
