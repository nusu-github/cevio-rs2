use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=.windows/winmd/CeVIO.Talk.RemoteService2.winmd");
    println!("cargo:rerun-if-changed=build.rs");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    let warnings = windows_bindgen::bindgen([
        "--in",
        "default",
        ".windows/winmd/CeVIO.Talk.RemoteService2.winmd",
        "--out",
        out_path.to_str().unwrap(),
        "--filter",
        "CeVIO.Talk.RemoteService2",
        "--reference",
        "windows,skip-root,Windows",
        "--no-allow"
    ]);

    warnings.iter().for_each(|warning| {
        println!("cargo:warning={warning}");
    });
}
