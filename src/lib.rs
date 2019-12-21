extern crate serde;
extern crate serde_json;

use reqwest::Client;
use serde_json::value::Value;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Bridge {
    /// The bridge struct represents a HUE bridge.
    /// The constructor for this struct `link`, tries to
    /// detect the light id's and is able to send new state to either
    /// a single light or all of the ones detected.
    ///
    /// Currently to register a new bridge, you need to know 2 things:
    /// - the internal IP of the bridge (in future i'll implement ssdp)
    /// - the key assigned to you by the `api/newdeveloper` POST request
    ///   see the documentation for the HUE API in order to see this.
    ///   I will implement the authentication handshake request flow in
    ///   the near future and in the CLI I will cache the key somewhere
    ///
    pub ip: String,
    pub key: String,
    pub client: Client,
    pub base_url: String,
    pub light_ids: Vec<u8>,
    pub n_lights: u8,
}

impl Bridge {
    /// Struct constructor that sets up the required interactions
    /// and also gets us the light id's as well as how many there are
    ///
    pub fn link(ip: String, key: String) -> Self {
        let client = Client::new();
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
