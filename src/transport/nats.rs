#[derive(Debug)]
pub struct NatsTransport {
    hosts: Vec<String>,
    nats_conn: nats::Connection,
}

use crate::{transport, config};

impl transport::Transport for NatsTransport {
    
    fn send(&self, data: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

use nats;

impl NatsTransport {
    pub fn build(cfg: &config::Config) -> NatsTransport {
        let nc = nats::connect("localhost").expect("Unable to connect to nats");
        NatsTransport{hosts: cfg.transport_broker_hosts(), nats_conn: nc}
    }
}

