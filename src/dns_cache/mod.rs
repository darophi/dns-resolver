use std::collections::HashMap;

use crate::resolver::DnsQuery;

pub struct DnsCache {
    cache: HashMap<String, DnsQuery>,
}

impl DnsCache {
    pub fn new() -> DnsCache {
        DnsCache {
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, name: &String) -> Option<DnsQuery> {
        let res = self.cache.get(name).map(|s| s.clone());

        match res {
            Some(msg) => {
                let ttl = msg.message.answers.first().to_owned().unwrap().ttl;

                if ttl > 0 {
                    println!("TTL: {}", ttl);
                    return Some(msg);
                }
                None
            }
            None => None,
        }
    }

    pub fn set(&mut self, name: &str, msg: &DnsQuery) {
        self.cache.insert(name.to_string(), msg.clone());
    }
}
