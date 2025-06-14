use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
pub struct OPAInput<'a> {
    input: OPARequest<'a>,
}

#[derive(Serialize)]
pub struct OPARequest<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub user: &'a str,
}

#[derive(Deserialize)]
pub struct OPAResponse {
    pub result: bool,
}

pub async fn check_policy(method: &str, path: &str, user: &str) -> bool {
    let input = OPAInput {
        input: OPARequest {
            method,
            path,
            user,
        },
    };

    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:8181/v1/data/http/authz/allow")
        .json(&input)
        .send()
        .await;

    match resp {
        Ok(r) => {
            let policy: OPAResponse = r.json().await.unwrap_or(OPAResponse { result: false });
            policy.result
        }
        Err(_) => false,
    }
}
