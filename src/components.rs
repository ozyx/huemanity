use crate::command::*;
use crate::network::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Bridge {
    pub ip: String,
    pub key: Option<String>,
}

impl std::fmt::Display for Bridge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ip:{}\nkey:{:?}", self.ip, self.key)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Light {
    pub state: LightState,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LightState {
    on: bool,
    bri: u8,
    hue: u32,
    sat: u8,
    effect: String,
    xy: (f32, f32),
    ct: u32,
    alert: String,
    colormode: String,
    mode: String,
    reachable: bool,
}

impl LightState {
    pub fn toggle(&mut self) {
        let _new_on = match self.on {
            false => true,
            true => false,
        };
    }
}
