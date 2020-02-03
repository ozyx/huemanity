#[macro_use]
extern crate clap;
extern crate serde_json;
use huemanity::{bridge::*, lightstructs::*};

fn main() {
    let matches = clap_app!(myapp =>
             (version: "0.1.0")
             (author: "Art Eidukas <iwiivi@gmail.com>")
             (about: "Given HUE bridge credentials, allows control over your HUE lights")
             (@subcommand info =>
                 (about: "Prints out the state of the lights that the bridge can detect")
             )
             (@subcommand all =>
                 (about: "Sends commands to all lights")
                 (@arg STATE: -s --state +takes_value "Takes a string input representing a new state to send to ALL lights")
             )
         )
         .get_matches();

    let bridge = Bridge::link();

    // resolve commands
    if let Some(matches) = matches.subcommand_matches("all") {
        if let Some(jsn) = matches.value_of("STATE") {
            let parsed = serde_json::from_str::<SendableState>(jsn);
            match parsed {
                Ok(state) => {
                    if let Err(e) = bridge.state_all(&state) {
                        println!("Could not send state: {}", e);
                    };
                }
                Err(e) => println!("Error in parsing state: {}", e),
            }
        }
    } else if let Some(_) = matches.subcommand_matches("info") {
        bridge.light_info();
    }
}
// TODO: add premate commands on the sendablestate
// TODO: implement the clap cli
//     // TODO: add file tracking
// TODO: make command sending async as this is quite inconsistent -> prerelease reqwests // Use stream
// TODO: add translation of color spaces to whatever the format in the API states
// TODO: add ssdp ip detection
// TODO: add a nice way to print out information about the system or lights and maybe dump it
// TODO: add usage of structs for state
// TODO: remove nasty unwraps
// TODO: add blink function
