extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate rmp_serialize;
extern crate rmpv;
extern crate rustc_serialize;

use rmp_serialize::{Decoder, Encoder};
use rmps::{Deserializer, Serializer};
use rustc_serialize::{Decodable, Encodable};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

fn main() {
    println!("Hello, world!");
    test1();
    //test2();
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
struct Custom {
    id: u32,
    key: String,
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
struct Custom2 {
    ID: u32,
    KEY: String,
}

fn test2() {
    println!("\ntest2 start");
    let val = Custom {
        id: 42u32,
        key: "the Answer".to_string(),
    };

    let mut buf = [0u8; 13];

    val.encode(&mut Encoder::new(&mut &mut buf[..]));

    println!("buf = {:?}", buf);

    let mut decoder = Decoder::new(&buf[..]);
    let res: Custom2 = Decodable::decode(&mut decoder).ok().unwrap();

    println!("res = {:?}", res);
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Human {
    age: u32,
    name: String,

    #[serde(default, rename = "test")]
    test: String,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
struct Human2 {
    //#[serde(rename = "age")]
    #[serde(default)]
    my_age: u32,

    //#[serde(default)]
    //my_id: u32,
    #[serde(default)]
    name: String,

    #[serde(default, rename = "addr")]
    addr: String,

    lala: String,
}

fn test1() {
    println!("test1 start");
    let mut buf = Vec::new();
    let val = Human {
        age: 42,
        name: "John".into(),
        test: "test".into(),
    };
    val.serialize(&mut Serializer::with(
        &mut buf,
        rmps::encode::StructMapWriter,
    ))
    .unwrap();
    println!("buf = {:?}", buf);

    let cur = Cursor::new(&buf[..]);
    let mut de = Deserializer::new(cur);
    let out: Human2 = Deserialize::deserialize(&mut de).unwrap();
    println!("out = {:?}", out);
}
