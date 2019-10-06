#![feature(proc_macro_hygiene)]
extern crate phf;
extern crate phf_macros;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod PubMessage;
pub mod PubCommandDef;


use phf::phf_map;


