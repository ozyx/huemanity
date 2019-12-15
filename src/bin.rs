mod lib;

use lib::*;
use std::env;

use serde_json::json;

fn main() {
    dotenv::dotenv().ok();
    // let on = match env::var("SW").unwrap().as_ref() {
    //     "ON" => true,
    //     "OFF" => false,
    //     _ => false,
    // };

    let ip = env::var("IP").unwrap();
    let key = env::var("KEY").unwrap();

    let bridge = Bridge::link(ip, key);

    // TODO: make name detection automatic
    // TODO: add usage of structs for state
    bridge.state_mult(vec![1, 2, 3], &json!({"on":false, "transitiontime":1}));
    std::thread::sleep(std::time::Duration::from_secs(1));
    bridge.state_mult(vec![1, 2, 3], &json!({"on":true, "transitiontime":1}));
}
