mod lib;

#[macro_use]
extern crate clap;
extern crate serde_json;

use lib::*;
use std::env;

use serde_json::{json, value::Value};

fn main() {
    dotenv::dotenv().ok();

    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Art Eidukas <iwiivi@gmail.com>")
        (about: "Given HUE bridge credentials, allows control over your HUE lights")
        (@subcommand all =>
            (about: "Sends commands to all lights")
            (version: "0.1.0")
            (@arg STATE: -s --state +takes_value "Takes a string input representing a new state to send to ALL lights")
        )
    )
    .get_matches();

    let mut bridge = Bridge::link();

    // resolve commands
    if let Some(matches) = matches.subcommand_matches("all") {
        if let Some(jsn) = matches.value_of("STATE") {
            let parsed = serde_json::from_str::<Value>(jsn);
            match parsed {
                Ok(state) => {
                    bridge.state_all(&state);
                }
                Err(e) => println!("Error in parsing state: {}", e),
            }
        }
    }
}
// TODO: implement the clap cli
//     // TODO: add file tracking
// TODO: make command sending async as this is quite inconsistent -> prerelease reqwests // Use stream
// TODO: add functionality to detect ip and register new app automatically
// TODO: add translation of color spaces to whatever the format in the API states
// TODO: at least add the registration if not the ssdp ip detection
// TODO: add a nice way to print out information about the system or lights and maybe dump it
// TODO: add usage of structs for state
// TODO: remove nasty unwraps
// TODO: add blink function
