use std::env;

pub fn initial_logger() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
}
