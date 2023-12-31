use autometrics::autometrics;
use tokio::spawn;
use tonic::{transport::Server, codec::CompressionEncoding, Request, Response, Status, async_trait, Code};
use crate::{
  CONFIG,
  proto::{*, authentication_service_server::{AuthenticationService, AuthenticationServiceServer}},
  THREAD_CANCELLATION_TOKEN,
  domain::usecases::Usecases, utils::SERVER_ERROR
};
use tracing::instrument;

const MAX_REQUEST_SIZE: usize= 512; //bytes

pub struct GrpcAdapter { }

impl GrpcAdapter {
  // startServer starts a gRPC server.
  pub async fn startServer(usecases: Box<Usecases>) {
    let address= format!("[::]:{}", &*CONFIG.GRPC_SERVER_PORT);
    let address= address.parse( )
                        .expect(&format!("ERROR: parsing binding address of the gRPC server : {}", address));

    let authenticationService= AuthenticationServiceServer::new(AuthenticationServiceImpl{ usecases })
      .max_decoding_message_size(MAX_REQUEST_SIZE)
      .send_compressed(CompressionEncoding::Gzip)
      .accept_compressed(CompressionEncoding::Gzip);

    let reflectionService= tonic_reflection::server::Builder::configure( )
      .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
      .build( )
      .expect("Error building gRPC reflection service")
      .max_decoding_message_size(MAX_REQUEST_SIZE);

    println!("INFO: Starting gRPC server");

    spawn(async move {
      Server::builder( )
        .add_service(authenticationService)
        .add_service(reflectionService)
        .serve_with_shutdown(address, THREAD_CANCELLATION_TOKEN.clone( ).cancelled( ))
        .await.expect("ERROR: starting gRPC server");
    });
  }
}

struct AuthenticationServiceImpl {
  usecases: Box<Usecases>
}

#[async_trait]
impl AuthenticationService for AuthenticationServiceImpl {

  #[instrument(name = "StartRegistration", skip(self))]
  #[autometrics]
  async fn start_registration(&self, request: Request<StartRegistrationRequest>) ->  Result<Response<( )> ,Status> {
    let request= request.into_inner( );

    self.usecases.startRegistration(&request).await
      .map(|_| Response::new(( )))
      .map_err(mapToGrpcError)
  }

  #[instrument(name = "VerifyUser", skip(self))]
  #[autometrics]
  async fn verify_user(&self, request: Request<VerifyUserRequest>) ->  Result<Response<( )> ,Status> {
    let request= request.into_inner( );

    self.usecases.verifyUser(&request).await
      .map(|_| Response::new(( )))
      .map_err(mapToGrpcError)
  }

  #[instrument(name = "Signin", skip(self))]
  #[autometrics]
  async fn signin(&self, request: Request<SigninRequest>) ->  Result<Response<SigninResponse> ,Status> {
    let request= request.into_inner( );

    self.usecases.signin(&request).await
      .map(|output| Response::new(SigninResponse { jwt: output.jwt }))
      .map_err(mapToGrpcError)
  }
}

// mapToGrpcError takes an anyhow error, analyses the actual underlying error and returns an
// appropriate gRPC status code.
fn mapToGrpcError(error: anyhow::Error) -> Status {
  let errorAsString= error.to_string( );

  let grpcErrorCode=
    if errorAsString.eq(SERVER_ERROR) { Code::Internal }
    else { Code::InvalidArgument };

  Status::new(grpcErrorCode, errorAsString)
}