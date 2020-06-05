#[derive(Debug)]
pub struct Config {
    listen_address: String,
    port: u32,
}

impl Config {
    pub fn load() -> Config {
        Config { listen_address : String::from("0.0.0.0"), port : 8088}
    }
    
    pub fn port(&self) -> u32 {
        self.port
    }

    pub fn listen_address(&self) -> &str {
        &self.listen_address
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.listen_address(), self.port())
    }

    pub fn transport_broker_hosts(&self) -> Vec<String> {
        vec!["localhost".to_string()]
    }
}
