use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: Option<String>,
    pub aud: Option<String>,
    pub iss: Option<String>,
    pub iat: Option<usize>,
    pub nbf: Option<usize>,
}

pub fn process_jwt_sign(opts: Claims) -> anyhow::Result<String> {
    let sig = encode(
        &Header::default(),
        &opts,
        &EncodingKey::from_secret(b"secret"),
    )?;
    Ok(sig)
}

pub fn process_jwt_verify(token: &str) -> anyhow::Result<bool> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["device1", "device2", "device3"]);
    let token = decode::<Claims>(token, &DecodingKey::from_secret(b"secret"), &validation);
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let flag = token.unwrap().claims.exp > (current_time as usize);
    println!("{:?}", flag);
    Ok(flag)
}
