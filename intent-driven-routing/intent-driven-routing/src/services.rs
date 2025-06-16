pub fn process_service_a(mission: &str) -> String {
    format!("Service A picked for mission '{}': fast, cost-effective route.", mission)
}

pub fn process_service_b(mission: &str) -> String {
    format!("Service B picked for mission '{}': premium, high-availability route.", mission)
}

pub fn process_service_c(mission: &str) -> String {
    format!("Service C picked for mission '{}': experimental, test traffic.", mission)
}
