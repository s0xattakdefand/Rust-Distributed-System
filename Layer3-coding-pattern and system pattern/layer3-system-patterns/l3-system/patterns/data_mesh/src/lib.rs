//! Data-Mesh: domain emits a Protobuf schema and other services decode it.

include!(concat!(env!("OUT_DIR"), "/user.rs"));  // `User` struct generated here

use anyhow::Result;
use prost::Message;

/// Round-trip encode → decode to prove schema compatibility.
pub fn roundtrip(u: User) -> Result<User> {
    let mut buf = Vec::new();
    u.encode(&mut buf)?;
    Ok(User::decode(&*buf)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn proto_roundtrip() {
        let orig = User { id: 7, name: "alice".into() };
        let out  = roundtrip(orig.clone()).unwrap();
        assert_eq!(orig.name, out.name);
    }
}
