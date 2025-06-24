//! Synchronous policy check stub (no Tokio needed).

/// Allow only admin* actions for user "admin".
pub fn allow(user: &str, action: &str) -> bool {
    user == "admin" && action.starts_with("admin:")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ok() {
        assert!(allow("admin", "admin:read"));
        assert!(!allow("bob", "admin:read"));
    }
}
