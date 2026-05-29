extern crate bindgen;
extern crate cmake;

use std::path::Path;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let enet_dir = Path::new("vendor/enet");

    println!("cargo:return-if-changed={}", enet_dir.display());

    let mut build = cc::Build::new();
    build.include(enet_dir.join("include"))
        .warnings(false)
        .define("ENET_STATIC", None);

    let core_files = [
        "address.c",
        "callbacks.c",
        "compress.c",
        "host.c",
        "list.c",
        "packet.c",
        "peer.c",
        "protocol.c",
    ];

    for file in core_files { build.file(enet_dir.join(file)); }

    if target.contains("windows") {
        build.file(enet_dir.join("win32.c"));
        println!("cargo:rustc-link-lib=dylib=winmm");
        build.define("WIN32", None);
    }
    else {
        build.file(enet_dir.join("unix.c"));
    }

    build.compile("enet");

    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/enet/include/")
        .header("wrapper.h")
        .derive_debug(false)
        .blocklist_type("ENetPacket")
        .blocklist_type("_ENetPacket")
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs")).expect("Coulnd't create bindings!");
}
