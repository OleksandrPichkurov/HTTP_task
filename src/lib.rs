use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::path::Path;
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
}

pub fn run(port : String, folder : String) {

    let addrs = [
        SocketAddr::from((Ipv4Addr::LOCALHOST, port.parse::<u16>().unwrap()),),
        SocketAddr::from((Ipv6Addr::LOCALHOST, port.parse::<u16>().unwrap())),
    ];

    let listener = TcpListener::bind(&addrs[..]).expect("Try to bind to other port");
    
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let folder = folder.clone();
        let stream = stream.unwrap();

        pool.execute( || {
            handle_connection(stream, folder);
        });
    }
}

fn handle_connection(mut stream: TcpStream, folder: String) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let main_page = b"GET / HTTP/1.1\r\n";
    let path = Path::new(&folder);
    let root_folder = env::home_dir().unwrap();
    let path = path.join(root_folder).join(path);

    let folder = format!("GET /{} HTTP/1.1\r\n", folder);


    let (status_line, filename) = if buffer.starts_with(main_page) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(folder.as_bytes()) {
        ("HTTP/1.1 200 OK", "")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    if filename.len() == 0 {
        let contents = fs::read(path).unwrap();
        let response = format!(
            "{}\r\nContent-Disposition: attachment\r\nContent-Length: {}\r\n\r\n",
            status_line,
            contents.len(),
        );
        stream.write(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
        stream.flush().unwrap();
    } else {
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}