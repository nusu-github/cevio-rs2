fn main() {
    println!("cargo:rerun-if-changed=.windows/winmd/CeVIO.Talk.RemoteService2.winmd");
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::bindgen([
        "--in",
        ".windows/winmd/CeVIO.Talk.RemoteService2.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "CeVIO.Talk.RemoteService2",
        "--config",
        "implement",
        "--config",
        "vtbl",
    ])
    .unwrap();
}
