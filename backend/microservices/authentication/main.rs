#![allow(non_snake_case)]

mod proto {
  // Including code generated from the .proto files.

  tonic::include_proto!("authentication.microservice");

  // Descriptors are the commonly used language model for Protocol Buffers. They are used as an
  // intermediate artifact to support code generation, and they are also used in runtime libraries
  // to implement support for reflection and dynamic types.
  // Read more here - https://protobuf.com/docs/descriptors
  pub const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("authentication-microservice.descriptor");
}

mod utils;
mod adapters;
mod domain;

use std::process::exit;
use adapters::{SurrealdbAdapter, GrpcAdapter};
use domain::usecases::Usecases;
use lazy_static::lazy_static;
use tokio::{spawn, signal};
use tokio_util::sync::CancellationToken;

pub struct Config {
  pub JWT_SECRET: String,
  pub GRPC_SERVER_PORT: String,
  pub SURREALDB_URL: String,
  pub SURREALDB_PASSWORD: String
}

lazy_static! {
  // This value is initialized (in a thread safe manner) on the heap, when it is accessed for the
  // first time.
  // Read more about lazy_static here - https://blog.logrocket.com/rust-lazy-static-pattern/
  pub static ref CONFIG: Config= {

    // Load environment variables from a .env file, during development process.
    if let Err(error)= dotenv::from_filename("./backend/microservices/authentication/.env.dev") {
      println!("WARNING: error loading environment variables from .env.dev : {}", error)
    }

    Config {
      JWT_SECRET: utils::getEnv("JWT_SECRET"),
      GRPC_SERVER_PORT: utils::getEnv("GRPC_SERVER_PORT"),
      SURREALDB_URL: utils::getEnv("SURREALDB_URL"),
      SURREALDB_PASSWORD: utils::getEnv("SURREALDB_PASSWORD")
    }
  };

  // This cancellation token will be activated when the program receives a shutdown signal. It will
  // trigger cleanup tasks in active Tokio threads.
  pub static ref THREAD_CANCELLATION_TOKEN: CancellationToken= CancellationToken::new( );
}

// Under the hood, Tokio creates a runtime which manages threads and IO resources. It submits the
// future representing your main function to the tokio runtime executor. The tokio executor calls
// the poll method on that future.
#[tokio::main] // By default, Tokio will spawn a separate thread to run the Tokio runtime.
async fn main( ) {
  let surrealdbAdapter= Box::new(SurrealdbAdapter::new( ).await);

  let usecases= Box::new(Usecases::new(surrealdbAdapter));

  spawn(async move {
    GrpcAdapter::startServer(usecases).await;
  });

  /* Gracefully shutdown on receiving program shutdown signal. */ {
    let error= signal::ctrl_c( ).await.err( );
    println!("Received program shutdown signal");

    let _ =&THREAD_CANCELLATION_TOKEN.cancel( ); // Do cleanup tasks in currently active Tokio
                                                 // threads.

    match error {
      None => exit(0),

      Some(error) => {
        println!("Error: {}", error);
        exit(1);
      }
    }
  }
}