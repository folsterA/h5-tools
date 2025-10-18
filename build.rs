use std::{env, fs::File, io::Write, path::PathBuf, process::Command};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_path).unwrap();

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=C:/Program Files/HDF_Group/HDF5/1.14.6/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=hdf5");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // point to headers needed for includes
        .clang_arg("-IC:/Program Files/HDF_Group/HDF5/1.14.6/include")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // 3) Emit C helper to print the constant
    let c_src = out_path.join("get_native_double.c");
    let mut f = File::create(&c_src)
        .unwrap_or_else(|_| panic!("Couldn't create C file at {}", c_src.display()));
    write!(
        f,
        r#"
        #include <stdio.h>
        #include <hdf5.h>

        int main() {{
            H5open();
            printf("%lld", (long long)H5T_NATIVE_DOUBLE_g);
            return 0;
        }}
    "#
    )
    .unwrap();

    // force Clang—even on Windows
    let compiler = "clang"; // or "clang-cl" if you prefer the MSVC-ABI frontend

    let exe = out_path.join("get_native_double.exe");
    let output = Command::new(compiler)
        .arg(&c_src)
        .arg("-I")
        .arg(r"C:\Program Files\HDF_Group\HDF5\1.14.6\include")
        .arg("-L")
        .arg(r"C:\Program Files\HDF_Group\HDF5\1.14.6\lib")
        .arg("-lhdf5")
        .arg("-o")
        .arg(&exe)
        .output()
        .expect("failed to invoke clang");

    println!(
        "cargo:warning=clang stdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!(
        "cargo:warning=clang stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    if !output.status.success() {
        panic!("clang exited with {}", output.status);
    }

    // 6) Run it and capture the numeric value
    // let output = Command::new(&exe).output().expect("Failed to run helper");
    let val = String::from_utf8(output.stdout).unwrap().trim().to_string();

    // 7) Write a small Rust file with the real const
    let const_rs = out_path.join("h5_native_consts.rs");
    let mut out = File::create(&const_rs).unwrap();
    writeln!(
        out,
        "/// baked at build-time via H5open() and H5T_NATIVE_DOUBLE_g\n\
         pub const H5T_NATIVE_DOUBLE: hid_t = {};",
        val
    )
    .unwrap();
}
