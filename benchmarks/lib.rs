pub mod all_cores;
pub mod single_core;

use std::env;

fn main() {
    let env_key = "RUST_LOG";
    env::set_var(env_key, "info");

    single_core::max_single_core_usage();
}
