fn main() {
    println!("cargo:rerun-if-changed=.windows/winmd/CeVIO.Talk.RemoteService2.winmd");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-if-changed=src/bindings.rs");

    let warnings = windows_bindgen::bindgen([
        "--in",
        "default",
        ".windows/winmd/CeVIO.Talk.RemoteService2.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "CeVIO.Talk.RemoteService2",
        "--reference",
        "windows,skip-root,Windows",
    ]);

    warnings.iter().for_each(|warning| {
        println!("cargo:warning={warning}");
    });
}
