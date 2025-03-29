

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/calculator.proto")?;
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/calculator.proto"], &["proto/"])?;
    Ok(())
}

