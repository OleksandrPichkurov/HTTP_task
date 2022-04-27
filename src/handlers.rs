use std::net::TcpStream;
use std::fs;
use std::path::Path;
use std::io::prelude::*;

use crate::resource::Resource;
use crate::logs::log;

const OK:&str = "HTTP/1.1 200 OK";
const NOT_FOUND:&str = "HTTP/1.1 404 Not Found";
const BAD_REQUEST:&str = "HTTP/1.1 400 Bad Request";

pub fn handle_connection(mut stream: TcpStream, folder: &String) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let main_page = b"GET / HTTP/1.1\r\n";
    let resource = Resource {path: Path::new(&folder)};
    let ff = String::from_utf8_lossy(&buffer[resource.path.to_str().unwrap().len()+4..]);
    let file_name = ff.split_whitespace().next().unwrap();
    let ss = format!("GET {}{} HTTP/1.1\r\n", &folder, file_name);
    let mut filename = false;

    let response = if buffer.starts_with(main_page) {
        handle_response(OK, "hello.html")
    } else if buffer.starts_with(ss.as_bytes()){
        filename = true;
        format!(
            "{}\r\nContent-Disposition: attachment\r\nContent-Length:\r\n\r\n", OK
        )
    } else {
        log("url shoud start with root path with value wich you pass to params".to_string());
        handle_response(BAD_REQUEST, "400.html")
    };

    if filename {
        let vec_of_folders  = resource.folder_contents().unwrap();
        if let Some(path_to_file) = vec_of_folders.iter().position(|x| x.ends_with(file_name)){
            let contents = fs::read(vec_of_folders.get(path_to_file).unwrap()).unwrap();
            stream.write(response.as_bytes()).unwrap();
            stream.write_all(&contents).unwrap();
            stream.flush().unwrap()
        } else {
            let response = handle_response(NOT_FOUND, "404.html");
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap()
        }
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_response(status_line: &str, filename: &str) -> String{
    if filename.len() > 0{
        let contents = fs::read_to_string(filename).unwrap();
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        )
    } else {
        format!(
            "{}\r\nContent-Disposition: attachment\r\nContent-Length: \r\n\r\n", OK
        )
    }
}