use std::{
    collections::HashSet,
    io::{self, Read, Result, Write},
    net::TcpStream,
    env
};

use ffi::Event;
use poll::Poll;

mod ffi;
mod poll;

fn main() {
    println!("Hello, world!");
}

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}
