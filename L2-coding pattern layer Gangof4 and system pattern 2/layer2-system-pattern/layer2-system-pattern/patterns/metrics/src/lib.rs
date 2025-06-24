use metrics::{counter, describe_counter, Unit};

pub fn init() {
    describe_counter!("hits", Unit::Count, "Total hits");
}
pub fn hit() {
    counter!("hits", 1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn inc() { super::hit(); }
}
