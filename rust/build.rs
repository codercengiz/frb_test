//! This build script is responsible for the Dart and FFI code generation.
//!
//! Everything in here could theoretically also be done via the
//! `flutter_rust_bridge_codegen` commandline tool, but it's pretty cumbersome.
//! 
//! 
use lib_flutter_rust_bridge_codegen::codegen;
use lib_flutter_rust_bridge_codegen::codegen::Config;
use lib_flutter_rust_bridge_codegen::utils::logs::configure_opinionated_logging;

fn main() -> anyhow::Result<()> {
    println!("cargo:warning=Running build script...");
    println!("cargo:rerun-if-changed=src/api");


    configure_opinionated_logging("./logs/", true)?;

    let config= Config::from_config_file("../flutter_rust_bridge.yaml")?.unwrap();

    codegen::generate(
        config,
        Default::default(),
    )
}