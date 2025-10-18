use std::{env, path::PathBuf};

fn main() {
    // 1) Compile our shim.c
    cc::Build::new()
        .file("shim.c")
        .include(r"C:\Users\Austin\Documents\hdf5-1.14.6\install114\my-Static-Tools-Clang\include")
        .compile("shim");

    // 2) Generate bindings for all H5 symbols + our shims
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(
            r"-IC:\Users\Austin\Documents\hdf5-1.14.6\install114\my-Static-Tools-Clang\include",
        )
        .clang_arg("-Wl,/DEF:exports.def")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("H5.*") // all H5Tget_native_type, H5Tcreate, etc.
        .allowlist_type("H5.*")
        .allowlist_var("H5.*")
        .generate()
        .expect("Unable to generate HDF5 bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out.join("bindings.rs")).unwrap();

    // 3) Link with HDF5 and our shim
    println!(
        "cargo:rustc-link-search=native=C:/Users/Austin/Documents/hdf5-1.14.6/install114/my-Static-Tools-Clang/lib"
    );
    println!("cargo:rustc-link-lib=static=hdf5");
    println!("cargo:rustc-link-lib=static=shim");
}
