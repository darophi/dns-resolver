use std::collections::HashMap;

pub struct IpCache {
    cache: HashMap<String, String>,
}

impl IpCache {
    pub fn new() -> IpCache {
        IpCache {
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<String> {
        self.cache.get(name).map(|s| s.to_string())
    }

    pub fn set(&mut self, name: String, ip: &str) {
        self.cache.insert(name.to_string(), ip.to_string());
    }
}
