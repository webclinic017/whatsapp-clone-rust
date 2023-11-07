use std::env;
use anyhow::anyhow;

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