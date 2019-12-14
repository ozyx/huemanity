use crate::components::*;
use crate::network::*;
use serde_json::json;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

pub struct Info;

impl Command for Info {
    fn generate_request(&self, key: Option<&String>) -> Request {
        Request {
            request_type: RequestType::Get,
            uri: "api/newdeveloper".to_owned(),
            params: None,
        }
    }
}

pub struct SystemState;

impl Command for SystemState {
    fn generate_request(&self, key: Option<&String>) -> Request {
        Request {
            request_type: RequestType::Get,
            uri: format!("api/{}/lights", key.unwrap()),
            params: None,
        }
    }
}

pub struct Register {
    pub username: String,
    pub appname: String,
}

impl Command for Register {
    fn generate_request(&self, key: Option<&String>) -> Request {
        let mut map: HashMap<String, String> = HashMap::new();
        // TODO: too many conversions
        map.insert(
            "devicetype".to_string(),
            format!("{}#{}", self.appname, self.username),
        );

        Request {
            request_type: RequestType::Post,
            uri: "api".to_owned(),
            params: Some(map),
        }
    }
    fn run_on(&self, bridge: &mut Bridge) -> Response {
        let check = |r: &Response| r.body[0]["error"]["type"] == json!(101);
        let mut resp = self.send(bridge);

        while check(&resp) {
            println!("Please press the Bridge Button");
            sleep(Duration::from_secs(10));
            resp = self.send(bridge);
        }

        println!("Button press detected!");
        if bridge.key.is_none() {
            let key = &resp.body[0]["success"]["username"];
            bridge.key = key.as_str().map(|s| s.to_owned());
        }
        resp
    }
}

pub trait Command {
    fn generate_request(&self, key: Option<&String>) -> Request;
    fn run_on(&self, bridge: &mut Bridge) -> Response {
        let response = self.send(bridge);
        println!("{}", response);
        response
    }
    fn send(&self, bridge: &mut Bridge) -> Response {
        let key = bridge.key.as_ref();
        let request = self.generate_request(key);
        let uri: String = format!("http://{}/{}", bridge.ip, request.uri);
        match request.request_type {
            RequestType::Post => {
                let client = reqwest::Client::new();
                // unsafe unwrapping
                let params = &request.params;
                let mut res = client
                    .post(&uri)
                    .json(&params.as_ref().unwrap())
                    .send()
                    .unwrap();
                Response {
                    code: res.status(),
                    body: res.json().unwrap(),
                }
            }
            RequestType::Get => {
                let mut res = reqwest::get(&uri).unwrap();
                Response {
                    code: res.status(),
                    body: res.json().unwrap(),
                }
            }
            RequestType::Put => {
                let client = reqwest::Client::new();
                // unsafe unwrapping
                let params = &request.params;
                let mut res = client
                    .put(&uri)
                    .json(&params.as_ref().unwrap())
                    .send()
                    .unwrap();
                Response {
                    code: res.status(),
                    body: res.json().unwrap(),
                }
            }
        }
    }
}
