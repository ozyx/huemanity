extern crate serde;
extern crate serde_json;

use reqwest::Client;
use serde_json::value::Value;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Bridge {
    /// The bridge struct represents a HUE bridge.
    /// The constructor for this struct `link`, tries to
    /// detect the light id's and is able to send new state to either
    /// a single light or all of the ones detected.
    ///
    /// Currently to register a new bridge, you need to know the internal IP of the bridge
    /// on the network. If you know that use the register method to actually register a key
    /// your app.
    ///
    /// If you already have a key and IP use the `link` method to create a bridge. However you
    /// must make sure tha the HUE_IP and HUE_KEY environment variables are set or the `.env` file
    /// in the executables directory has these variables set.
    ///
    pub ip: String,
    pub key: String,
    pub client: Client,
    pub base_url: String,
    pub light_ids: Vec<u8>,
    pub n_lights: u8,
}

impl Bridge {
    /// Discovers if a HUE_IP and HUE_KEY are available in the environment
    fn discover() -> Result<(String, String), Box<dyn Error>> {
        let ip = env::var("HUE_IP")?;
        let key = env::var("HUE_KEY")?;
        Ok((ip, key))
    }

    /// Register the Bridge and save credentials to .env file
    /// Can be used as a standalone function to get a key registered
    /// if you know the IP of your Bridge, but generally is a helper for
    /// the `link` method.
    pub fn register() -> Result<(String, String), Box<dyn Error>> {
        // TODO: currently uses file writting rather than some more clever serialisation and checking
        // TODO: could check if .env exists but has not been loaded before going  through the process
        // TODO: could also take an optional setting string or config path ?
        // TODO: stop hardcoding this
        let _client = Client::new();
        let mut ip = String::new();
        let mut name = String::new();

        // Get user IP input and name for the app
        println!("NOTE! Registration will create the `.env` containing IP and KEY info");
        println!("Enter the IP of your HUE bridge:");
        std::io::stdin().read_line(&mut ip)?;
        ip = ip.trim().to_string();

        println!("Enter the desired app name:");
        std::io::stdin().read_line(&mut name)?;
        name = name.trim().to_string();

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

        let mut file = File::create(".env")?;
        file.write_all(format!("HUE_IP=\"{}\"\nHUE_KEY={}", ip, key).as_ref())?;
        println!(".env File successfully saved!");

        // TODO: hacky replace
        Ok((ip, key.to_string().replace("\"", "")))
    }

    /// Struct constructor that sets up the required interactions
    /// and also gets us the light id's as well as how many there are
    ///
    /// If you have `HUE_IP` and `HUE_KEY` in your environment this will
    /// just proceed as normal linking to the bridge. If you don't have these
    /// variables in the environment, it will try to guide you throught a registration
    /// process. In that case you will need to know the IP of your Bridge.
    pub fn link() -> Self {
        let client = Client::new();

        let (ip, key) = match Self::discover() {
            Ok((ip, key)) => (ip, key),
            _ => {
                println!("Unable to find required `HUE_KEY` and `HUE_IP` in environment!");
                // TODO: can fail here, should check
                Self::register().unwrap()
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
        };

        // collect the id's it can find on the network
        // TODO: handle this to show that no lights were found
        bridge.collect_ids();

        // figure out how many lights we have
        bridge.n_lights = bridge.light_ids.len() as u8;
        println!("Connected to:\n{}", bridge);
        println!("Found {} lights", bridge.n_lights);
        bridge
    }

    /// Sends the a request with set parameters to the HUE API endpoint
    pub fn send(
        &self,
        endpoint: &str,
        req_type: RequestType,
        params: Option<&serde_json::value::Value>,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let target = format!("{}{}", self.base_url, endpoint);
        let response = match req_type {
            RequestType::Post => self.client.post(&target).json(&params).send()?,
            RequestType::Get => self.client.get(&target).send()?,
            RequestType::Put => self.client.put(&target).json(&params).send()?,
        };
        Ok(response)
    }

    /// Given a light and a required state, send this state to the light.
    pub fn state(&self, light: u8, state: &Value) -> Result<(), Box<dyn std::error::Error>> {
        self.send(
            &format!("lights/{}/state", light),
            RequestType::Put,
            Some(state),
        )?;
        Ok(())
    }

    /// Given a state send it to all lights found on bridge
    pub fn state_all(&self, state: &Value) -> Result<(), Box<dyn std::error::Error>> {
        for light in self.light_ids.iter() {
            self.state(*light, state)?;
        }
        Ok(())
    }

    /// Collect all found light ids
    pub fn collect_ids(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = "lights";
        let map: HashMap<u8, Value> = self.send(&endpoint, RequestType::Get, None)?.json()?;
        let ids: Vec<u8> = map.keys().cloned().map(|integer| integer).collect();
        self.light_ids = ids;
        Ok(())
    }

    /// Currently this function only prints out the light_ids, but there is scope in future
    /// for this to print out more. I will have to refactor the `collect_ids` into collect info
    /// and serialize all the data about the ligths (perhaps except state which will change).
    pub fn light_info(&self) {
        // TODO: get info about the ligths and serialize all that along with the ids
        // NOTE: will make this more useful as now you can't tell which light is which
        // TODO: make it sorted
        println!("Lights available on your bridge");

        for id in self.light_ids.iter() {
            println!("{}", id)
        }
    }
}

impl fmt::Display for Bridge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bridge: {}\nlights: {:?}", self.ip, self.light_ids)
    }
}

// TODO: stop using propriatary enums etc, reuse the ones from Reqwest
#[derive(PartialEq)]
pub enum RequestType {
    Get,
    Post,
    Put,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Light {
//     pub state: LightState,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct LightState {
//     pub on: bool,
//     pub bri: u8,
//     pub hue: u32,
//     pub sat: u8,
//     pub effect: String,
//     pub xy: (f32, f32),
//     pub ct: u32,
//     pub alert: String,
//     pub colormode: String,
//     pub mode: String,
//     pub reachable: bool,
// }
