use std::collections::HashSet;

pub struct Firewall {
    pub blocked_ips: HashSet<String>,
}

impl Firewall {
    pub fn new() -> Self {
        Self {
            blocked_ips: HashSet::new(),
        }
    }

    pub fn is_blocked(&self, ip: &str) -> bool {
        self.blocked_ips.contains(ip)
    }

    pub fn block_ip(&mut self, ip: String) {
        self.blocked_ips.insert(ip);
    }

    pub fn unblock_ip(&mut self, ip: &str) {
        self.blocked_ips.remove(ip);
    }
}
