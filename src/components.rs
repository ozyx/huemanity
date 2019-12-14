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
