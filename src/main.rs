#[macro_use]
extern crate log;
extern crate env_logger;

// mod servers;
// mod services;
mod services;

fn main() {
    env_logger::init();
    // let _ = servers::run_server();
    // services::process_wordcount();
    services::provide_text();
}

