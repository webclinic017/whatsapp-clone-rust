use autometrics::autometrics;
use tokio::spawn;
use tonic::{transport::Server, codec::CompressionEncoding, Status, Code, Request, Response};
use async_trait::async_trait;
use crate::{
  CONFIG,
  proto::{*, profile_service_server::{ProfileService, ProfileServiceServer}},
  THREAD_CANCELLATION_TOKEN,
  domain::{usecases::Usecases, ports}, utils::SERVER_ERROR
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

    let profileService= ProfileServiceServer::new(ProfileServiceImpl{ usecases })
      .max_decoding_message_size(MAX_REQUEST_SIZE)
      .send_compressed(CompressionEncoding::Gzip)
      .accept_compressed(CompressionEncoding::Gzip);

    let reflectionService= tonic_reflection::server::Builder::configure( )
      .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
      .build( )
      .expect("Error building gRPC reflection service")
      .max_decoding_message_size(MAX_REQUEST_SIZE);

    println!("Starting gRPC server");

    spawn(async move {
      Server::builder( )
        .add_service(profileService)
        .add_service(reflectionService)
        .serve_with_shutdown(address, THREAD_CANCELLATION_TOKEN.clone( ).cancelled( ))
        .await.expect("ERROR: starting gRPC server");
    });
  }
}

struct ProfileServiceImpl {
  usecases: Box<Usecases>
}

#[async_trait]
impl ProfileService for ProfileServiceImpl {

  #[instrument(name = "SearchProfiles", skip(self))]
  #[autometrics]
  async fn search_profiles(&self, request: Request<SearchProfilesRequest>) -> Result<Response<SearchProfilesResponse>, Status> {
    let request= request.into_inner( );

    self.usecases.searchProfiles(&request.query).await
      .map(|profiles| {

        let profiles= profiles.iter( )
                              .map(|profile| transformProfileDataType(profile))
                              .collect( );

        Response::new(SearchProfilesResponse { profiles })
      })
      .map_err(mapToGrpcError)
  }

  #[instrument(name = "GetProfileByUserId", skip(self))]
  #[autometrics]
  async fn get_profile_by_user_id(&self, request: Request<GetProfileByUserIdRequest>) -> Result<Response<GetProfileByUserIdResponse>, Status> {
    let request= request.into_inner( );

    self.usecases.getProfileByUserId(&request.user_id).await
      .map(|profile| {
        Response::new(GetProfileByUserIdResponse {
          profile: Some(transformProfileDataType(&profile))
        })
      })
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

// transformProfileDataType is used to transform ports::Profile to Profile.
fn transformProfileDataType(profile: &ports::Profile) -> Profile {
  Profile {
    id: profile.id.to_string( ),
    user_id: profile.userId.to_owned( ),
    name: profile.name.to_owned( ),
    username: profile.username.to_owned( )
  }
}