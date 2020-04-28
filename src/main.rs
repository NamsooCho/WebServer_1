#[macro_use]
extern crate log;
extern crate env_logger;

// mod servers;
mod services;

fn main() {
    crate::env_logger::init();
    // servers::run_server().expect("failed to start server");
    // services::process_wordcount();
    services::provide_text();
}

