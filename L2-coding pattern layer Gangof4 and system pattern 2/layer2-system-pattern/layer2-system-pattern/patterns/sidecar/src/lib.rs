//! Sidecar / Ambassador pattern stub.

pub struct SidecarConfig {
    pub enable_tls: bool,
}

impl SidecarConfig {
    pub async fn start_proxy(&self) {
        println!("Starting sidecar. TLS={}", self.enable_tls);
        // TODO: launch Envoy or custom Rust proxy here.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cfg_struct() {
        let c = SidecarConfig { enable_tls: true };
        assert!(c.enable_tls);
    }
}
