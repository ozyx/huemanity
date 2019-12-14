use http;
use std::collections::HashMap;
use std::fmt::Display;

pub enum RequestType {
    Get,
    Post,
    Put,
}

pub struct Request {
    pub request_type: RequestType,
    pub uri: String,
    pub params: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Response {
    pub code: http::status::StatusCode,
    pub body: serde_json::value::Value,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "code: {}\nbody:\n{}",
            self.code,
            serde_json::to_string_pretty(&self.body).unwrap()
        )
    }
}
