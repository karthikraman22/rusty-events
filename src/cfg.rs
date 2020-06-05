pub mod gateway {

    pub struct Config {
        listen_address: String,
        port: u32,
    }

    impl Config {
        pub fn port(&self) -> u32 {
            self.port
        }
        pub fn listen_address(&self) -> &str {
            &self.listen_address
        }
    }
}