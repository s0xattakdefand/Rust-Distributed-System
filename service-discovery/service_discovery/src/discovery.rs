use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::service::ServiceInfo;

pub type SharedRegistry = Arc<Mutex<ServiceRegistry>>;

#[derive(Debug, Default)]
pub struct ServiceRegistry {
    services: HashMap<String, ServiceInfo>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register(&mut self, service: ServiceInfo) {
        self.services.insert(service.name.clone(), service);
    }

    pub fn discover(&self, name: &str) -> Option<&ServiceInfo> {
        self.services.get(name)
    }
}
