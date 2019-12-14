mod commands;
mod components;
mod network;

use commands::*;
use components::Bridge;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::collections::HashMap;

use dotenv;
use std::env;

fn main() {
    dotenv::dotenv().ok();
    let mut bridge = Bridge {
        ip: env::var("IP").unwrap(),
        key: Some(env::var("KEY").unwrap()),
    };

    if bridge.key.is_none() {
        let command = Register {
            username: "artie_fartie".to_string(),
            appname: "hue_stuff_rust".to_string(),
        };
        println!("{}", command.run_on(&mut bridge));
        println!("Registered on bridge:\n{}", bridge);
    }

    // TODO: do i need to use instance methods?

    let command = LightNames;
    command.retrieve_show(&mut bridge)
}

#[derive(Serialize, Deserialize)]
struct LightState {
    num: u8,
    on: bool,
    bri: u8,
    hue: u32,
    sat: u8,
    effect: String,
    xy: (f32, f32),
    ct: u32,
    alert: String,
    colormode: String,
    mode: String,
    reachable: bool,
}
