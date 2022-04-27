use std::env;
use std::process;

mod tcp_server;
mod param;
mod thread;
mod resource;
mod handlers;
mod logs;

use crate::tcp_server::Server;
use crate::param::Params;

fn main() {
    let params = Params::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    Server::run(params);

}