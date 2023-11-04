use std::env;
use anyhow::{Result, anyhow};
use chrono::Local;
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};
use crate::CONFIG;

pub const SERVER_ERROR: &'static str= "Server error occurred";

// getEnv fetches the environment variable with the given name.
pub fn getEnv(name: &str) -> String {
  env::var(name)
    .expect(&format!("Error getting env {}", name))
}

// toServerError captures any error, logs it and then returns SERVER_ERROR as an anyhow error.
pub fn toServerError(error: impl std::error::Error) -> anyhow::Error {
  println!("ERROR: {}", error.to_string( ));
  anyhow!(SERVER_ERROR)
}

// generateOtp generates a random 6 digit OTP.
pub fn generateOtp( ) -> String {
  thread_rng( )
    .gen_range(100000..999999)
    .to_string( )
}

// The JWT is structured as a set of claims (JSON key-value pairs) that provide information about
// the entity. There are three types of claims: Registered, Public and Private.
#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {

  // Registered Claims - standardized by the community.
  sub: String,
  issuedAt: usize,
  expiresAt: usize

  // Public and Private Claims.
}

impl JwtClaims {
  // new creates a new JwtClaim.
  fn new(sub: String) -> Self {
    let currentTimestamp= Local::now( ).timestamp( ) as usize;

    Self {
      sub,
      issuedAt: currentTimestamp,
      expiresAt: currentTimestamp + (60 * 60 * 12) // JWT expires after 12 hours.
    }
  }
}

lazy_static! {
  static ref JWT_ENCODING_KEY: EncodingKey= EncodingKey::from_secret(CONFIG.JWT_SECRET.as_bytes( ));
  static ref JWT_DECODING_KEY: DecodingKey= DecodingKey::from_secret(CONFIG.JWT_SECRET.as_bytes( ));
}

// createJwt is used to create a JWT.
pub fn createJwt(id: String) -> Result<String> {
  let claims= JwtClaims::new(id);

  encode(&Header::default( ), &claims, &JWT_ENCODING_KEY)
    .map_err(toServerError)
}

// decodeJwt
pub fn decodeJwt(jwt: &str) -> Result<String> {
  let tokenData= decode::<JwtClaims>(jwt, &JWT_DECODING_KEY, &Validation::default( ))
                          .map_err(toServerError)?;
  let claims= tokenData.claims;

  if claims.expiresAt < Local::now( ).timestamp( ) as usize {
    return Err(anyhow!("JWT expired"))}

  Ok(claims.sub)
}