use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = vec![env::var("SERVICE_PROTO").unwrap()];

    for proto in protos {
        tonic_build::compile_protos(proto)?;
    }

    Ok(())
}
