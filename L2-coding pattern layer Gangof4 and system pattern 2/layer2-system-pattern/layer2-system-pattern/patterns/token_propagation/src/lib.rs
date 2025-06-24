//! Encode + decode transit JWT; here HS256 via jsonwebtoken.

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

static SECRET: &[u8] = b"supersecret";

#[derive(Serialize, Deserialize)]
pub struct Claims { pub sub: String, pub exp: usize }

pub fn sign(sub:&str, exp:usize)->String{
    encode(&Header::default(),&Claims{sub:sub.into(),exp},&EncodingKey::from_secret(SECRET)).unwrap()
}
pub fn verify(tok:&str)->bool{
    decode::<Claims>(tok,&DecodingKey::from_secret(SECRET),&Validation::default()).is_ok()
}

#[cfg(test)]mod tests{use super::*;#[test]fn round(){let t=sign("u",999999);assert!(verify(&t));}}
