mod lib;

extern crate serde_json;

use lib::*;
use std::env;

use serde_json::json;

fn main() {
    dotenv::dotenv().ok();

    let ip = env::var("HUE_IP").unwrap();
    let key = env::var("HUE_KEY").unwrap();
    // let bri: u16 = env::var("BRI").unwrap().parse().unwrap();

    let bridge = Bridge::link(ip, key);

    println!("{:?}", bridge);
    bridge.state_all(&json!({"xy":[0.0,0.0],"transitiontime":1}));
}

// TODO: add usage of structs for state
