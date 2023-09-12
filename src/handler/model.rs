use lazy_static::lazy_static;
use std::collections::HashMap;
use parking_lot::RwLock;
lazy_static! {
    pub static ref REDIS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
    pub static ref CHANNELS: RwLock<HashMap<String,Vec<String>>> = RwLock::new(HashMap::new());
}
