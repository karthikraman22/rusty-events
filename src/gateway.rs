use crate::config;
use crate::transport::nats;

pub struct RouteManager {
    instance_name: String,
}

pub fn bootstrap(cfg: &config::Config) {
    println!("Boot strapping server {:?}", cfg);
    let transport = nats::NatsTransport::build(cfg);
    println!("Transport is {:?}", transport);
}

