pub mod nats;
 
pub trait Transport {
    fn send(&self, data: &str) -> std::io::Result<bool>; 
} 