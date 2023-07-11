use std::{env, path::PathBuf};

use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        // .out_dir("./proto/v1/") // if prefer to check-in generated artefacts
        .file_descriptor_set_path(out_dir.join("auction_v1_descriptor.bin"))
        .compile(
            &[
                "./proto/v1/model.proto",
                "./proto/v1/processor-service.proto",
                "./proto/v1/querier-service.proto",
            ],
            &["proto/v1"], // location to search proto dependencies
        )?;

    Ok(())
}
