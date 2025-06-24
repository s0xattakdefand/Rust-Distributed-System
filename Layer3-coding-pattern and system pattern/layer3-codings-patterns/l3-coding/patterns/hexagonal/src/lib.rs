//! Hexagonal = Domain-inside, Adapters-outside.
//
// Domain traits (`Port`s) live here; adapters in sibling modules/crates implement them.

pub trait PaymentPort {
    fn pay(&self, amount: u32) -> anyhow::Result<()>;
}

// ── domain service ───────────────────────────────────────────────────────────
pub struct Checkout<'a, P: PaymentPort> {
    gateway: &'a P,
}

impl<'a, P: PaymentPort> Checkout<'a, P> {
    pub fn new(gateway: &'a P) -> Self { Self { gateway } }

    pub fn buy(&self, total: u32) -> anyhow::Result<()> {
        self.gateway.pay(total)
    }
}

// ── tests with in-memory adapter ─────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    struct Dummy; impl PaymentPort for Dummy {
        fn pay(&self, _: u32) -> anyhow::Result<()> { Ok(()) }
    }

    #[test]
    fn pays_ok() {
        let c = Checkout::new(&Dummy);
        assert!(c.buy(100).is_ok());
    }
}
