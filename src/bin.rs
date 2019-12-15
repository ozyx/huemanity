mod commands;
mod components;
mod network;

use commands::*;
use components::*;
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

    // TODO: do i need to use instance methods? most likely yes

    let command = LightNames;
    command.retrieve_show(&mut bridge);

    let command = GetLights;
    let lights = command.retrieve(&mut bridge);
}

fn turn_on_all(lights: HashMap<u8, Light>) {
    for (which, light) in lights.iter() {
        let string_state = serde_json::to_string(&light.state);
        let new: HashMap<String, String> =
            serde_json::from_str(string_state.as_ref().unwrap()).unwrap();
    }
}
