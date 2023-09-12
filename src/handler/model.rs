use lazy_static::lazy_static;
use std::collections::HashMap;
use parking_lot::RwLock;
lazy_static! {
    pub static ref REDIS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}
