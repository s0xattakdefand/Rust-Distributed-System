use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConsulService {
    pub Service: String,
    pub Address: String,
    pub Port: u16,
}

pub async fn discover_service(name: &str) -> Option<String> {
    let url = format!("http://localhost:8500/v1/catalog/service/{}", name);
    let client = Client::new();
    match client.get(&url).send().await {
        Ok(res) => match res.json::<Vec<ConsulService>>().await {
            Ok(services) if !services.is_empty() => {
                let s = &services[0];
                Some(format!("http://{}:{}", s.Address, s.Port))
            }
            _ => None,
        },
        Err(_) => None,
    }
}
