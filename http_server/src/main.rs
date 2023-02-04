use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

const IP_ADDR: &str = "127.0.0.1";
const PORT: &str = "8080";

static HTTP_200_OK: &'static str = String::from("");

fn main() {
    let addr = format!("{}:{}", IP_ADDR, PORT);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        handle_http_connection(stream.unwrap());
    }
}

fn handle_http_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}

fn http_200_ok() {}
