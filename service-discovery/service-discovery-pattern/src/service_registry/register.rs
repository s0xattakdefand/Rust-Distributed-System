use serde::Serialize;
use reqwest::Client;

#[derive(Serialize)]
struct ConsulRegistration {
    ID: String,
    Name: String,
    Address: String,
    Port: u16,
}

pub async fn register_to_consul(name: &str, address: &str, port: u16) {
    let registration = ConsulRegistration {
        ID: name.to_string(),
        Name: name.to_string(),
        Address: address.to_string(),
        Port: port,
    };

    let client = Client::new();
    let url = "http://localhost:8500/v1/agent/service/register";
    let res = client.put(url).json(&registration).send().await;

    match res {
        Ok(response) if response.status().is_success() => {
            println!("✅ '{}' registered to Consul at {}:{}", name, address, port);
        }
        Ok(response) => {
            eprintln!("❌ Failed to register: {:?}", response.text().await);
        }
        Err(e) => eprintln!("❌ Error registering to Consul: {:?}", e),
    }
}
