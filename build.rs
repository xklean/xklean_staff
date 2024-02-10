fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file_staff = "./server/xklean_staff.proto";
    //let proto_file_message = "./server/message.proto";

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("./server/")
        .compile(&[proto_file_staff],&["."])
        .unwrap_or_else(|e|panic!("protobuf compile error{}",e));
    Ok(())
}

