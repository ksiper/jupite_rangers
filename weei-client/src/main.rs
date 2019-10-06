use std::io::{self, Read};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

pub mod protocol;

use protocol::cli_proto::*;

#[macro_use]
extern crate serde;
extern crate pub_proto;
extern crate rmp_serde as rmps;
extern crate serde_json;

use pub_proto::PubCommandDef::*;
use pub_proto::PubMessage::*;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

fn main() {
    let client_stream = TcpStream::connect("127.0.0.1:21178").expect("Couldcn't connect");
    let mut writer = BufWriter::new(&client_stream);

    let a = P2pMessage { x: 1, y: 2 };
    let b = new_b(32, 53);

    let c = MetaMessage { a: 1, b: 2 };

    println!("{}", b);
    println!("{}", c);

    let serialized = serde_json::to_string(&c).unwrap();
    println!("serialized to string: {}", serialized);
    println!("{:?}", serialized.as_bytes());

    let st: &str = &serialized[..];
    let v: Value = serde_json::from_str(st).unwrap();
    println!("{:?}", v["a"]);

    writer.write(serialized.as_bytes()).expect("error");
    writer.flush();

    repl_loop();
}

fn repl_loop() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);

        // remove end of line '\n'
        let len = buffer.len();
        buffer.truncate(len - 1);

        // Parse command for operation
        command_eval(buffer);
    }
}

fn command_eval(command: String) {
    println!("{}", command);
    match &command[..] {
        "create_account" => println!("{}", COMMAND_MAP.get("create_account").cloned().unwrap()),
        _ => println!("{}", "nothing"),
    }
}
