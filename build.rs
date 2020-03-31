fn main() {
    println!("cargo:rerun-if-changed={}", "src/test.proto");
    println!("cargo:rerun-if-changed={}", "src/test.rs");

    tonic_build::configure()
        .out_dir("src")
        .format(false)
        .build_client(true)
        .build_server(true)
        .compile(&["src/test.proto".to_string()], &["src".to_string()])
        .unwrap();
}
