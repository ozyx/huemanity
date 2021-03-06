#[macro_use]
extern crate clap;
extern crate serde_json;
use huemanity::{bridge::*, lightstructs::*};

// ssdp
extern crate ssdp;

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
             (@subcommand debug =>
                 (about: "Send a get request to the bridge and return the raw response")
             )
             (@subcommand search =>
                 (about: "Search for a bridge and print out the IP of the bridge")
             )
             (@subcommand discover =>
                 (about: "Discover the bridges on the network (NOTE: EXPERIMENTAL)")
             )
             (@subcommand clean =>
                 (about: "Cleanup the `~/.huemanity` file")
             )
         )
         .get_matches();

    // resolve commands
    if let Some(matches) = matches.subcommand_matches("all") {
        if let Some(jsn) = matches.value_of("STATE") {
            let parsed = serde_json::from_str::<SendableState>(jsn);
            match parsed {
                Ok(state) => {
                    let bridge = Bridge::link();
                    if let Err(e) = bridge.state_all(&state) {
                        println!("Could not send state: {}", e);
                    };
                }
                Err(e) => println!("Error in parsing state: {}", e),
            }
        }
    } else if let Some(_) = matches.subcommand_matches("info") {
        let bridge = Bridge::link();
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
                        let bridge = Bridge::link();
                        if let Err(e) = bridge.state(lightid, &sendablestate) {
                            println!("Could not send state to light: {}", e);
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    } else if let Some(_) = matches.subcommand_matches("debug") {
        let bridge = Bridge::link();
        bridge.debug();

    // NOTE: The following subcommands don't need a bridge
    } else if let Some(_) = matches.subcommand_matches("discover") {
        println!("Discovered bridges on the following IPs: {:?}", discover());
    } else if let Some(_) = matches.subcommand_matches("clean") {
        match cleanup() {
            Ok(_) => println!("Cleaned up!"),
            Err(e) => println!("Could not clean up because: {}", e),
        }
    }
}

// TODO: stop printing the bloody bridge thing every time
// TODO: mapper between names or integer values to be kept in bridge
// TODO: add premate commands on the sendablestate
// TODO: implement the clap cli
//     // TODO: add file tracking
// TODO: make command sending async as this is quite inconsistent -> prerelease reqwests // Use stream
// TODO: add translation of color spaces to whatever the format in the API states
// TODO: add a nice way to print out information about the system or lights and maybe dump it
// TODO: add usage of structs for state
// TODO: remove nasty unwraps
// TODO: add blink function
// TODO: Refactor CLI
// TODO: implement more serialisation things like XY vs HS etc.
