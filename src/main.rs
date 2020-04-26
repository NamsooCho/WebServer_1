#[macro_use]
extern crate log;
extern crate env_logger;

// mod servers;
mod wc;

fn main() {
    env_logger::init();
    // let _ = servers::run_server();
    wc::process();
}

