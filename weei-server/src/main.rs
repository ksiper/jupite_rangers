use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::*;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

#[macro_use]
extern crate serde;
extern crate pub_proto;
extern crate rmp_serde as rmps;
extern crate serde_json;

use pub_proto::PubMessage::*;
use rmps::Deserializer;
use serde::Deserialize;
use serde_json::{Result, Value};

fn handle_client(stream: &mut TcpStream) {
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).unwrap();

    /// `res` is the exact bytes which was read by the listener
    for i in 0..buf.len() {
        println!("{}", buf[i]);
    }

    let cmd = str::from_utf8(&buf).unwrap();
    println!("{}", cmd);
    let v: Value = serde_json::from_str(cmd).unwrap();
    println!("{:?}", v["a"]);
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:21178")?;

    for stream in listener.incoming() {
        //let stream = match stream {
        //    Ok(val) => val,
        //    Err(err) => std::io::Error::new(ErrorKind::Other, "bad socket"),
        //};

        let mut stream = stream.unwrap();

        thread::spawn(move || {
            handle_client(&mut stream);
        });
    }
    Ok(())
}
