use std::collections::HashMap;

/**
 * server测试入口web
 * 2024.04.19 by dralee
 */
use draleewebmvc::server::{self, HttpServer};

fn main() {
    let mut server = server::Server::new();
    server.start(7880).unwrap();

    //demo();
}

fn demo() {
    let mut map = HashMap::new();
    map.insert("hello".to_string(), "good".to_string());
    map.insert("well".to_string(), "come".to_string());
    map.insert("how".to_string(), "are you".to_string());

    println!("has 'how': {}", map.contains_key("how"));
    if let Some(v) = map.get("well") {
        println!("the well value is {v}");
    }
    println!("size: {}", map.len());
}
