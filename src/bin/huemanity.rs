#[macro_use]
extern crate clap;
extern crate serde_json;
use huemanity::{bridge::*, lightstructs::*};

fn main() {
    let matches = clap_app!(huemanity =>
             (version: "0.1.0")
             (author: "Art Eidukas <iwiivi@gmail.com>")
             (about: "Given HUE bridge credentials, allows control over your HUE lights")
             (@subcommand info =>
                 (about: "Prints out the state of the lights that the bridge can detect")
             )
             (@subcommand state =>
                 (about: "Takes a string input (json, escaped quotes) of a new state and sends it to a given light")
                 (@arg STATE: +required "Takes a string input representing a new state to send to ALL lights")
                 (@arg LIGHT: +required "You need to provide the numerical ID of the light")

             )
             (@subcommand all =>
                 (about: "Sends commands to all lights")
                 (@arg STATE: +required "Takes a string input (json, escaped quotes) of a new state and sends it to all lights")
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
    } else if let Some(matches) = matches.subcommand_matches("state") {
        let state = matches.value_of("STATE");
        let light = matches.value_of("LIGHT");
        // TODO: Can we tidy this up?
        match (state, light) {
            (Some(state), Some(light)) => {
                match (
                    serde_json::from_str::<SendableState>(state),
                    light.parse::<u8>(),
                ) {
                    (Ok(sendablestate), Ok(lightid)) => {
                        bridge.state(lightid, &sendablestate);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

// TODO: mapper between names or integer values to be kept in bridge
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
