use std::env;
#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server; 

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
}
