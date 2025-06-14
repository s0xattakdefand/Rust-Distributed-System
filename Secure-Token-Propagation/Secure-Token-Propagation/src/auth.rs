use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use std::env;
use chrono::Utc; // ✅ Needed
use chrono::Duration; // ✅ Needed

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_token(user_id: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(10))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "defaultsecret".into());
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "defaultsecret".into());
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
}
