use std::env;
use lazy_static::lazy_static;

pub struct Config {
  pub GRPC_PORT: String
}

impl Config {
  // An instance of the Config struct is constructed using envs and returned back.
  fn init( ) -> Self {

    // During local development, the environment variables will be loaded from a .env file. But in
    // production environments, the environment variables will be injected to the container.
    if let Err(_)= dotenv::from_filename("backend/microservices/authentication/.env") {
      println!("Error loading .env file");
    }

    return Self {
      GRPC_PORT: getEnv("GRPC_PORT")
    };
  }
}

lazy_static! {
  // This value is initialized (in a thread safe manner) on the heap, when it is accessed for the
  // first time.
  // Read more about lazy_static here - https://blog.logrocket.com/rust-lazy-static-pattern/
  pub static ref CONFIG: Config= Config::init( );
}

// getEnv fetches the given environment variable.
fn getEnv(name: &str) -> String {
  return env::var(name)
    .expect(&format!("Error getting env {}", name));
}