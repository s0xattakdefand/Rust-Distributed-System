pub fn deploy_web_app(params: Option<&serde_json::Value>) -> Vec<String> {
    let name = params
        .and_then(|v| v.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("unnamed");
    vec![
        format!("Provision compute resources for app '{}'", name),
        format!("Deploy container image for '{}'", name),
        format!("Expose '{}':80 via load balancer", name),
        "Validate service health".to_string(),
        "Notify user of deployment success".to_string(),
    ]
}

pub fn scale_service(params: Option<&serde_json::Value>) -> Vec<String> {
    let name = params
        .and_then(|v| v.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("unnamed");
    let replicas = params
        .and_then(|v| v.get("replicas"))
        .and_then(|r| r.as_u64())
        .unwrap_or(1);
    vec![
        format!("Scale service '{}' to {} replicas", name, replicas),
        "Validate scaling event".to_string(),
        "Update monitoring dashboards".to_string(),
    ]
}

pub fn unknown_intent(goal: &str) -> Vec<String> {
    vec![format!(
        "No orchestration workflow found for intent '{}'",
        goal
    )]
}
