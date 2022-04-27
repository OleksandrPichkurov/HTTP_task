
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::env;
use std::num::ParseIntError;

pub struct Params {
    pub port: String,
    pub folder: String,
}

impl Params {
    pub fn new(mut args: env::Args) -> Result<Params, &'static str> {
        args.next();

        let port = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a port for listening"),
        };

        let folder = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a folder"),
        };

        Ok(Params {
            port,
            folder,
        })
    }
    pub fn get_addrs(&self) -> Result<Vec<std::net::SocketAddr>, ParseIntError> {
        Ok(vec![
        SocketAddr::from((Ipv4Addr::LOCALHOST, self.port.parse::<u16>()?)),
        SocketAddr::from((Ipv6Addr::LOCALHOST, self.port.parse::<u16>()?)),
    ])
    }
}

