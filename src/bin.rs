mod lib;

use lib::*;
use std::env;

use serde_json::json;

fn main() {
    dotenv::dotenv().ok();

    let ip = env::var("IP").unwrap();
    let key = env::var("KEY").unwrap();
    let bri: u16 = env::var("BRI").unwrap().parse().unwrap();

    let bridge = Bridge::link(ip, key);

    // TODO: make name detection automatic
    // TODO: add usage of structs for state
    bridge.state_mult(vec![1, 2, 3], &json!({"bri":bri,"transitiontime":1}));
}
