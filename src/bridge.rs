use crate::lightstructs::*;
use dotenv;
use reqwest::Client;
use serde_json::value::Value;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

type Lights = BTreeMap<u8, Light>;

/// The bridge struct represents a HUE bridge.
/// The constructor for this struct `link`, tries to
/// detect the lights and is able to send new state to either
/// a single light or all of the ones detected.
///
/// If you already have an application key and IP use the `link` method to create a bridge. However you
/// must make sure tha the HUE_IP and HUE_KEY environment variables are set in your environment
/// or the `.huemanity` file in the home directory has these variables set.
///
/// If you don't have the key registered yet, the link function will guide you through the
/// process to register the key and save it to the `.huemanity` file that will be loaded by the CLI
/// everytime.
#[derive(Debug)]
pub struct Bridge {
    ip: String,
    key: String,
    client: Client,
    base_url: String,
    pub light_ids: Vec<u8>,
    pub n_lights: u8,
    pub lights: Option<Lights>,
}

impl Bridge {
    /// Detects if a `HUE_IP` and `HUE_KEY` are available in the environment
    fn detect(filename: &str) -> Result<(String, String), Box<dyn Error>> {
        dotenv::from_filename(filename)?;
        let ip = env::var("HUE_IP")?;
        let key = env::var("HUE_KEY")?;
        Ok((ip, key))
    }
    /// Waits for a button to be pressed on a given bridge or several bridges
    fn wait_for_button(
        body: Value,
        ip: Option<&String>,
        ips: Option<Vec<String>>,
        client: Client,
    ) -> (String, String) {
        // needed to avoid repetition of code
        let ping_it = |i: &String| -> Value {
            client
                .post(&format!("http://{}/api", i))
                .json(&body)
                .send()
                .unwrap()
                .json()
                .unwrap()
        };

        // main logic. basically if you provide a single ip, it will
        // attempt to register to that
        // otherwise it will try to loop through the ips
        match (ip, ips) {
            (Some(bridge_ip), _) => {
                let mut response = ping_it(&bridge_ip);

                loop {
                    if response[0]["error"]["type"] == 101 {
                        println!("Please press the hub button!");
                        sleep(Duration::from_secs(5));
                        response = ping_it(&bridge_ip);
                    } else {
                        break;
                    }
                }

                (
                    bridge_ip.to_string(),
                    response[0]["success"]["username"].to_string(),
                )
            }
            (None, Some(ips)) => {
                let mut response: Value = Value::Bool(true);
                let mut bridge_ip = String::new();
                loop {
                    println!("Please press the hub button!");
                    sleep(Duration::from_secs(5));
                    // this chunk of code basically will loop through
                    // all the ips and check if any of them have the button
                    // pressed
                    for ip in &ips {
                        response = ping_it(&ip);
                        if response[0]["error"]["type"] == 101 {
                            continue;
                        } else {
                            bridge_ip.push_str(ip);
                            break;
                        }
                    }
                    if response[0]["error"]["type"] != 101 {
                        break;
                    }
                }

                (bridge_ip, response[0]["success"]["username"].to_string())
            }
            (None, None) => panic!("No ips provided in order to wait for a button press!"),
        }
    }
    /// Register the Bridge and save credentials to `~/.huemanity` file
    /// Can be used as a standalone function to get a key registered
    /// but the main use of this is through the `link` method.
    fn register(configpath: &str) -> Result<(String, String), Box<dyn Error>> {
        // TODO: currently uses file writting rather than some more clever serialisation and checking
        // TODO: could also take an optional setting string or config path ?

        // TODO: write a serialisation (serde) so one can load the bridge from config
        // Get user IP input and name for the app
        println!("NOTE! Registration will create the `~/.huemanity` containing IP and KEY info");
        let client = Client::new();
        let mut ip = String::new();
        let mut name = String::new();

        // Try to find bridges through ssdp
        let bridges = discover();

        println!("Enter the desired app name (default: huemanity):");
        std::io::stdin().read_line(&mut name)?;
        if name == "" {
            name = "huemanity".to_owned();
        } else {
            name = name.trim().to_string();
        }

        // only use json! here because its a one of and writing serialisation for it is pointless
        let body = serde_json::json!({ "devicetype": format!("{}", name) });

        // Deal with the cases where:
        // - bridge ip is not found
        // - mutliple bridges found
        // - one bridge found
        let (ip, key) = if bridges.len() == 0 {
            println!("No bridges automatically detected.\nEnter the IP of your HUE bridge (default: huemanity):");
            std::io::stdin().read_line(&mut ip)?;
            // TODO: use IP struct form net::sockaddr
            ip = ip.trim().to_string();
            Self::wait_for_button(body, Some(&ip), None, client)
        } else {
            println!(
                "Bridge(s) found: {:?} Will try to connect to all of them sequentially...",
                &bridges
            );
            Self::wait_for_button(body, None, Some(bridges), client)
        };

        let mut file = File::create(configpath)?;
        file.write_all(format!("HUE_IP=\"{}\"\nHUE_KEY={}", &ip, key).as_ref())?;
        println!(".huemanity File successfully saved!");

        // TODO: hacky replace
        Ok((ip, key.to_string().replace("\"", "")))
    }

    /// Struct constructor that sets up the required interactions
    /// and also gets us the lights that it can find on the system
    ///
    /// If you have `HUE_IP` and `HUE_KEY` in your environment this will
    /// just proceed as normal linking to the bridge. If you don't have these
    /// variables in the environment, it will try to guide you throught a registration
    /// process.
    ///
    /// It will attempt to find bridges on your network using UPnP. If it can't you will need
    /// to know the IP of your bridge and input it. If you have multiple bridges on the network
    /// it will try to connect to all of them until it finds one with a pressed Hub button.
    ///
    /// As part of the registration process it will also ask you for an app name. It is not
    /// really important what it is as it is used as an application identifier when you are
    /// trying to see which apps have been registered on your bridge.
    pub fn link() -> Self {
        let mut filename = dirs::home_dir().unwrap();
        filename.push(".huemanity");
        let path = filename.to_str().unwrap();

        let client = Client::new();

        // discovery of IP and registration logic
        let (ip, key) = match Self::detect(path) {
            Ok(tupl) => tupl,
            _ => {
                println!("Unable to find required `HUE_KEY` and `HUE_IP` in environment!");
                let result = match Self::register(path) {
                    Ok(tupl) => {
                        println!("Registration successful");
                        tupl
                    }
                    Err(e) => panic!("Could not register due to: {}", e),
                };
                result
            }
        };

        let base_url = format!("http://{}/api/{}/", ip, key);
        let mut bridge = Bridge {
            ip,
            key,
            client,
            base_url,
            light_ids: Vec::new(),
            n_lights: 0,
            lights: None,
        };

        // inform user we're connected
        println!("Connected to:\n{}", bridge);

        // collect the lights into the bridge
        match bridge.collect_lights() {
            Ok(_) => println!("Collected lights sucessfully!"),
            Err(e) => println!("Could not collect lights: {}", e),
        }

        println!("Found {} lights", bridge.n_lights);
        bridge
    }

    /// Sends the a request with set parameters to the HUE API endpoint
    /// This is a lower level function used primarily to send state.
    /// For more useful functions to look at: `Bridge.state` , `Bridge.state_all`
    fn send(
        &self,
        endpoint: &str,
        req_type: RequestType,
        params: Option<&SendableState>,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        // TODO: make it so it takes the state, and fills in the values from the same light
        let target = format!("{}{}", self.base_url, endpoint);
        let response = match req_type {
            RequestType::Post => self.client.post(&target).json(&params).send()?,
            RequestType::Get => self.client.get(&target).send()?,
            RequestType::Put => self.client.put(&target).json(&params).send()?,
        };
        Ok(response)
    }

    /// Gets the raw response to the user
    pub fn debug(&self) {
        match self.send("lights", RequestType::Get, None) {
            Ok(mut resp) => {
                let r: Value = resp.json().unwrap();
                println!("{}", serde_json::to_string_pretty(&r).unwrap());
            }
            Err(e) => {
                println!("Could not send the get request: {}", e);
            }
        };
    }

    /// Given a light and a required state, send this state to the light.
    pub fn state(
        &self,
        light: u8,
        state: &SendableState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement a threadpool solution where the pool is owned by the bridge and you
        // send light commands through that.
        self.send(
            &format!("lights/{}/state", light),
            RequestType::Put,
            Some(state),
        )?;
        Ok(())
    }

    /// Given a state send it to all lights found on bridge.
    /// At the moment it is done in a loop. So the lights don't get the
    /// signal sent concurrently
    pub fn state_all(&self, state: &SendableState) -> Result<(), Box<dyn std::error::Error>> {
        for light in self.light_ids.iter() {
            self.state(*light, state)?;
        }
        Ok(())
    }

    /// Collect all found light ids
    /// This method updates the following attributes of the bridge:
    /// - light_ids
    /// - n_lights
    /// - lights
    fn collect_lights(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // get the lights state
        let lights: Lights = self.send("lights", RequestType::Get, None)?.json()?;

        // update the values with the new ones
        self.light_ids = lights.keys().cloned().map(|integer| integer).collect();
        self.lights = Some(lights);
        self.n_lights = self.light_ids.len() as u8;

        Ok(())
    }

    /// This is a simple method to show the lights in the terminal
    pub fn light_info(&self) {
        // TODO: make a macro to nice print
        println!("--------------------------------");
        println!("Lights available on your bridge:");
        println!("--------------------------------");

        let lights = self.lights.as_ref().unwrap();
        for (id, light) in lights.iter() {
            println!("{}:{}", id, light);
        }
    }
}

impl fmt::Display for Bridge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bridge: {}", self.ip)
    }
}

/// Discovers bridge IPs on the networks using SSDP
pub fn discover() -> Vec<String> {
    println!("Searching for bridges...");
    use ssdp::header::{HeaderMut, Man, MX, ST};
    use ssdp::message::{Multicast, SearchRequest};

    // Create Our Search Request
    let mut request = SearchRequest::new();

    // Set Our Desired Headers (Not Verified By The Library)
    request.set(Man);
    request.set(MX(5));
    request.set(ST::Target(ssdp::FieldMap::URN(
        "urn:schemas-upnp-org:device:Basic:1".into(),
    )));

    let mut bridges = Vec::new();
    // Iterate Over Streaming Responses
    for (_, src) in request.multicast().unwrap() {
        let ip = src.ip().to_string();
        if !bridges.contains(&ip) {
            bridges.push(ip)
        }
    }
    bridges
}

/// Removes the `~/.huemanity` file
pub fn cleanup() -> std::io::Result<()> {
    // TODO: ideally remove this unwrap
    let mut filename = dirs::home_dir().unwrap();
    filename.push(".huemanity");
    std::fs::remove_file(filename)?;
    Ok(())
}
