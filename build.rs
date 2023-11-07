#![allow(non_snake_case)]

use std::{path::PathBuf, env};

fn main( ) {
  let outputDirectory = PathBuf::from(env::var("OUT_DIR").unwrap( ));

  let microservices= vec!{ "authentication", "profile" };

  for microservice in microservices {
    let protoFilePath= format!("protos/{}-microservice.proto", microservice);
    let descriptorFilePath= outputDirectory.join(format!("{}-microservice.descriptor.bin", microservice));

    tonic_build::configure( )
      .build_client(false)
      // Descriptors are the commonly used language model for Protocol Buffers. They are used as an
      // intermediate artifact to support code generation, and they are also used in runtime
      // libraries to implement support for reflection and dynamic types.
      // Read more here - https://protobuf.com/docs/descriptors
      .file_descriptor_set_path(descriptorFilePath)
      .compile(&[protoFilePath], &[ ] as &[&str])
    .unwrap( );
  }

  tonic_build::configure( )
    .compile(&["protos/events.proto"], &[ ] as &[&str])
    .unwrap( );
}