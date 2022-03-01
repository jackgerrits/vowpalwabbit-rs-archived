fn main() {
    // For some reason on Windows I had to force exception handling to be turned on.
    let exception_handling_flag = if cfg!(target_os = "windows") {
        "/EHsc"
    } else {
        ""
    };
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let dst = cmake::Config::new("binding")
        .define("BUILD_TESTING", "Off")
        .define("GIT_SUBMODULE", "Off")
        .define("WARNINGS", "Off")
        .define("USE_LATEST_STD", "On")
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", out_path.join("lib"))
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", out_path.join("lib"))
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY", out_path.join("bin"))
        .build_target("vw_rs_bindings")
        .cxxflag(exception_handling_flag)
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("bin").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("bin/Debug").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib/Debug").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("bin/Release").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib/Release").display()
    );
    println!("cargo:rustc-link-lib=vw_rs_bindings");

    let bindings = bindgen::Builder::default()
        .header("binding/src/binding.h")
        .generate()
        .unwrap();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
