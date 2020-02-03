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
    /// Discovers if a `HUE_IP` and `HUE_KEY` are available in the environment
    fn discover(filename: &str) -> Result<(String, String), Box<dyn Error>> {
        dotenv::from_filename(filename)?;
        let ip = env::var("HUE_IP")?;
        let key = env::var("HUE_KEY")?;
        Ok((ip, key))
    }

    /// Register the Bridge and save credentials to `~/.huemanity` file
    /// Can be used as a standalone function to get a key registered
    /// if you know the IP of your Bridge, but generally is a helper for
    /// the `link` method.
    pub fn register(configpath: &str) -> Result<(String, String), Box<dyn Error>> {
        // TODO: currently uses file writting rather than some more clever serialisation and checking
        // TODO: could also take an optional setting string or config path ?
        // TODO: write a serialisation (serde) so one can load the bridge from config
        let _client = Client::new();
        let mut ip = String::new();
        let mut name = String::new();

        // Get user IP input and name for the app
        println!("NOTE! Registration will create the `~/.huemanity` containing IP and KEY info");
        println!("Enter the IP of your HUE bridge:");
        std::io::stdin().read_line(&mut ip)?;
        ip = ip.trim().to_string();

        println!("Enter the desired app name:");
        std::io::stdin().read_line(&mut name)?;
        name = name.trim().to_string();

        // only use json! here because its a one of and writing serialisation for it is pointless
        let body = serde_json::json!({ "devicetype": format!("{}", name) });
        let ping_it = || -> Value {
            _client
                .post(&format!("http://{}/api", ip))
                .json(&body)
                .send()
                .unwrap()
                .json()
                .unwrap()
        };

        let mut response = ping_it();

        loop {
            if response[0]["error"]["type"] == 101 {
                println!("Please press the hub button!");
                sleep(Duration::from_secs(5));
                response = ping_it();
            } else {
                break;
            }
        }

        let key = &response[0]["success"]["username"];

        let mut file = File::create(configpath)?;
        file.write_all(format!("HUE_IP=\"{}\"\nHUE_KEY={}", ip, key).as_ref())?;
        println!(".env File successfully saved!");

        // TODO: hacky replace
        Ok((ip, key.to_string().replace("\"", "")))
    }

    /// Struct constructor that sets up the required interactions
    /// and also gets us the lights that it can find on the system
    ///
    /// If you have `HUE_IP` and `HUE_KEY` in your environment this will
    /// just proceed as normal linking to the bridge. If you don't have these
    /// variables in the environment, it will try to guide you throught a registration
    /// process. In that case you will still need to know the IP of your Bridge.
    pub fn link() -> Self {
        let mut filename = dirs::home_dir().unwrap();
        filename.push(".huemanity");
        let path = filename.to_str().unwrap();

        let client = Client::new();

        // discovery of IP and registration logic
        let (ip, key) = match Self::discover(path) {
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

        // collect the lights into the bridge
        match bridge.collect_lights() {
            Ok(_) => println!("Collected lights sucessfully!"),
            Err(e) => println!("Could not collect lights: {}", e),
        }

        println!("Connected to:\n{}", bridge);
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
    pub fn collect_lights(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
        // TODO: make it sorted
        println!("Lights available on your bridge:");

        let lights = self.lights.as_ref().unwrap();
        for (id, light) in lights.iter() {
            println!("{}:{}", id, light);
        }
    }
}

impl fmt::Display for Bridge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bridge: {}\nlights: {:?}", self.ip, self.light_ids)
    }
}
