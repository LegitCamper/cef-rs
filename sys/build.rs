use std::env;
use std::path::PathBuf;

fn main() {
    let path = match std::env::var("CEF_PATH") {
        Ok(val) => val,
        Err(_) => match std::env::var("HOME") {
            Ok(mut val) => {
                val.push_str("/.local/share/cef");
                val
            }
            Err(e) => panic!("Couldn't get the path of shared library: {e}"),
        },
    };

    println!("cargo:rustc-link-search={path}");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .allowlist_type("cef_.*")
        .allowlist_function("cef_.*")
        .bitfield_enum(".*_mask_t")
        .clang_arg(format!("-I{path}").as_str())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=cef");
}
