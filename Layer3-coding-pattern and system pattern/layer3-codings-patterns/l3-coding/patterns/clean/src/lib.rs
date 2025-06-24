//! Clean Architecture – entities → use-cases → interface.

//──── Entity ────
#[derive(Clone)]
pub struct User { pub id: u32, pub name: String }

//──── Boundary traits (interfaces) ────
pub trait UserRepo {
    fn find(&self, id: u32) -> anyhow::Result<Option<User>>;
}

//──── Use-case (interactor) ────
pub struct GetUser<'a, R: UserRepo> { repo: &'a R }
impl<'a, R: UserRepo> GetUser<'a, R> {
    pub fn exec(&self, id: u32) -> anyhow::Result<Option<User>> {
        self.repo.find(id)
    }
}

//──── Tests with in-memory repository ────
#[cfg(test)]
mod tests {
    use super::*;

    struct MemRepo(std::collections::HashMap<u32, User>);
    impl UserRepo for MemRepo {
        fn find(&self, id: u32) -> anyhow::Result<Option<User>> {
            Ok(self.0.get(&id).cloned())
        }
    }

    #[test]
    fn fetches() {
        let repo = MemRepo([(1, User { id:1, name:"bob".into() })].into());
        let uc   = GetUser{ repo:&repo };
        assert_eq!(uc.exec(1).unwrap().unwrap().name, "bob");
    }
}
