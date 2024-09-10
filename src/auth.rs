use anyhow::anyhow;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub exp: usize,
}

pub fn create_session_token(secret: &str, ttl_seconds: u64) -> anyhow::Result<String> {
    let now = Local::now().add(Duration::seconds(ttl_seconds as i64)).timestamp();

    let session_body = UserSession {
        exp: now as usize
    };

    let token = encode(
        &Header::new(Algorithm::HS256), &session_body,
        &EncodingKey::from_secret(secret.as_ref())
    )?;

    Ok(token)
}

/// Extract user session from JWT token
/// Return error for expired token
pub fn get_session_from_token(secret: &str, token: &str) -> anyhow::Result<UserSession> {
    let session = decode::<UserSession>(
        &token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()
    )?;

    let now = Local::now().timestamp();

    if now < session.claims.exp as i64 {
        info!("session is valid");
        Ok(session.claims)

    } else {
        info!("session is expired");
        Err(anyhow!("session is expired"))
    }
}