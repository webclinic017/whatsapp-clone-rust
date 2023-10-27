use std::time::Duration;
use anyhow::Error;
use tokio::{spawn, time::interval};
use tonic::Code;
use crate::proto::{*, authentication_service_server::*};
use crate::utils::{SERVER_ERROR, THREAD_CANCELLATION_TOKEN};
use crate::{config::CONFIG, domain::Usecases};
use tonic::{
  codec::CompressionEncoding,
  async_trait,
  Status,
  Response,
  Request,
  transport::Server
};

const MAX_REQUEST_SIZE: usize= 512; //bytes

pub struct GrpcAdapter { }

impl GrpcAdapter {

  // startServer creates a gRPC server and starts it at the desired network address (provided
  // via an environment variable).
  pub async fn startServer(&self, usecases: &'static Usecases) {
    let address= format!("[::]:{}", &*CONFIG.GRPC_PORT);
    let address= address.parse( )
                        .expect(&format!("Error parsing binding address of gRPC server : {}", address));

    let authenticationService= AuthenticationServiceServer::new(AuthenticationServiceImpl{ usecases })
      .max_decoding_message_size(MAX_REQUEST_SIZE)

      // Read more about the compression feature - https://grpc.io/docs/guides/compression/.
      .send_compressed(CompressionEncoding::Gzip)
      .accept_compressed(CompressionEncoding::Gzip);

    // Support for gRPC server health-checking.
    let (mut healthReporter, healthcheckService)= tonic_health::server::health_reporter( );
    spawn(async move {
      let mut ticker= interval(Duration::from_secs(5));
      loop {
        ticker.tick( ).await;

        let servingStatus= tonic_health::ServingStatus::Serving;
        healthReporter.set_service_status("AuthenticationService", servingStatus).await;
      }
    });

    // Adding gRPC server reflection capabilities.
    let reflectionService= tonic_reflection::server::Builder::configure( )
      .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
      .build( )
      .expect("Error building gRPC reflection service")
      .max_decoding_message_size(MAX_REQUEST_SIZE);

    println!("Starting gRPC server");

    Server::builder( )
      .add_service(authenticationService)
      .add_service(healthcheckService)
      .add_service(reflectionService)
      .serve_with_shutdown(address, THREAD_CANCELLATION_TOKEN.clone( ).cancelled( ))
      .await.expect("Error trying to start the gRPC server");
  }
}

#[derive(Debug)]
pub struct AuthenticationServiceImpl {
  usecases: &'static Usecases
}

#[async_trait]
impl AuthenticationService for AuthenticationServiceImpl {

  async fn say_hello(&self, request: Request<( )>) -> Result<Response<SayHelloReponse>, Status> {

    let response= SayHelloReponse{ message: "Hello".to_string( ) };
    return Ok(Response::new(response));
  }

}

// getGrpcStatusCode takes an anyhow error and returns an appropriate gRPC status code by analysing
// the error.
fn getGrpcStatusCode(error: &Error) -> Code {
  if error.to_string( ) == SERVER_ERROR {
    return Code::Internal;
  }

  else { return Code::InvalidArgument; }
}