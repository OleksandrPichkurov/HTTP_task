use std::env;
use std::process;

use web3::Params;

fn main() {
    let params = Params::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    web3::run(params.port, params.folder);
}