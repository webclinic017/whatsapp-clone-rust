#![allow(non_snake_case)]

use std::{path::PathBuf, env};

fn main( ) {
  let outputDirectory = PathBuf::from(env::var("OUT_DIR").unwrap( ));

  tonic_build::configure( )
    .build_client(false)
    .file_descriptor_set_path(outputDirectory.join("authentication-microservice.descriptor.bin"))
    .compile(&["protos/authentication-microservice.proto"], &[""])
  .unwrap( );
}