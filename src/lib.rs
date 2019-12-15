use serde::{Deserialize, Serialize};
use serde_json::{json, value::Value};

pub struct Bridge {
    pub ip: String,
    pub key: String,
    pub client: reqwest::Client,
    pub base_url: String,
}

impl Bridge {
    pub fn link(ip: String, key: String) -> Self {
        let client = reqwest::Client::new();
        let base_url = format!("http://{}/api/{}/", ip, key);
        Bridge {
            ip,
            key,
            client,
            base_url,
        }
    }

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

    pub fn state(&self, light: u8, state: &Value) -> Result<(), Box<dyn std::error::Error>> {
        self.send(
            &format!("lights/{}/state", light),
            RequestType::Put,
            Some(state),
        )?;
        Ok(())
    }

    pub fn state_mult(
        &self,
        lights: Vec<u8>,
        state: &Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for light in lights.iter() {
            self.state(*light, state)?;
        }
        Ok(())
    }
}

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
