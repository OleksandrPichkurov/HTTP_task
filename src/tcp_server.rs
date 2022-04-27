use std::net::TcpListener;

use crate::param::Params;
use crate::thread::ThreadPool;
use crate::handlers::handle_connection;
pub struct Server{}

impl Server {
    pub fn run(params: Params) {
        let addrs = match params.get_addrs(){
            Ok(addrs) => addrs,
            Err(err) => panic!("{}, Port number should be an integer", err),
        };

        
        let listener = TcpListener::bind(&addrs[..]).expect("Try to bind to other port");
        
        let pool = ThreadPool::new(4);
    

        for stream in listener.incoming() {
            let folder = params.folder.clone();
            pool.execute( move ||{
                handle_connection(stream.unwrap(), &folder);
            });
        }
    }
}