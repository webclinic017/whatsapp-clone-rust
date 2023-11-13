#![allow(non_snake_case)]

mod proto {
  // Including code generated from the .proto files.

  tonic::include_proto!("profile.microservice");
  tonic::include_proto!("events");

  pub const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("profile-microservice.descriptor");
}

mod utils;
mod adapters;
mod domain;

use std::process::exit;
use adapters::{SurrealdbAdapter, GrpcAdapter, MetricsServer, Tracer};
use domain::usecases::Usecases;
use lazy_static::lazy_static;
use tokio::signal;
use tokio_util::sync::CancellationToken;

pub struct Config {
  pub GRPC_SERVER_PORT: String,
  pub METRICS_SERVER_PORT: String,
  pub JAEGER_COLLECTOR_URL: String,
  pub SURREALDB_URL: String,
  pub SURREALDB_PASSWORD: String
}

lazy_static! {
  pub static ref CONFIG: Config= {

    // Load environment variables from a .env file, during development process.
    if let Err(error)= dotenv::from_filename("./backend/microservices/profile/.env.dev") {
      println!("WARNING: error loading environment variables from .env.dev : {}", error)
    }

    Config {
      METRICS_SERVER_PORT: utils::getEnv("METRICS_SERVER_PORT"),
      JAEGER_COLLECTOR_URL: utils::getEnv("JAEGER_COLLECTOR_URL"),
      GRPC_SERVER_PORT: utils::getEnv("GRPC_SERVER_PORT"),
      SURREALDB_URL: utils::getEnv("SURREALDB_URL"),
      SURREALDB_PASSWORD: utils::getEnv("SURREALDB_PASSWORD")
    }
  };

  pub static ref THREAD_CANCELLATION_TOKEN: CancellationToken= CancellationToken::new( );
}

#[tokio::main]
async fn main( ) {
  let surrealdbAdapter= Box::new(SurrealdbAdapter::new( ).await);

  let usecases= Box::new(Usecases::new(surrealdbAdapter));

  MetricsServer::new( ).await;
  Tracer::new( );

  GrpcAdapter::startServer(usecases).await;

  /* Gracefully shutdown on receiving program shutdown signal. */ {
    let error= signal::ctrl_c( ).await.err( );
    println!("Received program shutdown signal");

    let _ = &THREAD_CANCELLATION_TOKEN.cancel( ); // Do cleanup tasks in currently active Tokio
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