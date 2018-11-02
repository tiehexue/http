extern crate http;
extern crate rand;

use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::time::Duration;
use http::pool::ThreadPool;
use rand::Rng;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

  let pool = ThreadPool::new(4);

  for stream in listener.incoming().take(33) {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }

  println!("Going shutdown...");
}

fn handle_connection(mut stream: TcpStream) {

  let mut buffer = [0;512];
  stream.read(&mut buffer).unwrap();

  let request = String::from_utf8_lossy(&buffer);

  let content = if request.starts_with("GET / ") {
    read_file("index.html")
  } else {
    let sleep = rand::thread_rng().gen_range(1, 5);
    thread::sleep(Duration::from_secs(sleep));
    read_file("404.html")
  };

  stream.write(format!("HTTP/1.1 200 OK\r\n\r\n{}", content).as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn read_file(filename: &str) -> String {
  fs::read_to_string(format!("./resources/{}", filename)).unwrap()
}
