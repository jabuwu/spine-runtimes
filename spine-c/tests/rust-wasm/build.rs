use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let is_wasm = target.starts_with("wasm32");

    // Build spine-cpp with no-cpprt variant
    let spine_cpp_dir = PathBuf::from("../../../spine-cpp");
    let spine_c_dir = PathBuf::from("../..");

    let mut cpp_build = cc::Build::new();
    cpp_build
        .cpp(true)
        .include(spine_cpp_dir.join("include"))
        .include(spine_c_dir.join("include"))
        .include(spine_c_dir.join("src"))
        .define("SPINE_NO_FILE_IO", None)
        .define("HORRIBLE_RUST_WASM_HACK", None)
        .flag("-Wno-everything")
        .flag("-std=c++11");
    if is_wasm {
        cpp_build.include("./wasm-include");
    }

    // Always avoid C++ runtime (consistent with no-cpprt approach)
    cpp_build
        .flag("-fno-exceptions")
        .flag("-fno-rtti");

    // Always avoid C++ runtime (consistent with no-cpprt approach)
    cpp_build.flag("-nostdlib++");
    
    // Tell cc crate linker to not link libc++
    cpp_build.flag_if_supported("-Wl,-undefined,dynamic_lookup");
    cpp_build.cpp_link_stdlib(None);

    // Add spine-cpp source files (no-cpprt variant = all sources + no-cpprt.cpp)
    let spine_cpp_src = spine_cpp_dir.join("src");
    for entry in std::fs::read_dir(&spine_cpp_src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "cpp") {
            cpp_build.file(&path);
        }
    }

    // Add spine-cpp subdirectories
    for subdir in &["spine", "spine-c", "utils"] {
        let subdir_path = spine_cpp_src.join(subdir);
        if subdir_path.exists() {
            for entry in std::fs::read_dir(&subdir_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "cpp") {
                    cpp_build.file(&path);
                }
            }
        }
    }

    // Add spine-c generated sources
    let spine_c_generated = spine_c_dir.join("src/generated");
    for entry in std::fs::read_dir(&spine_c_generated).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "cpp") {
            cpp_build.file(&path);
        }
    }

    // Add spine-c extensions
    cpp_build.file(spine_c_dir.join("src/extensions.cpp"));

    // Add no-cpprt.cpp for C++ runtime stubs
    cpp_build.file(spine_cpp_dir.join("src/no-cpprt.cpp"));

    cpp_build.compile("spine");

    // Generate bindings with bindgen
    let bindings = bindgen::Builder::default()
        .header("../../include/spine-c.h")
        .clang_arg("-I../../include")
        .clang_arg("-I../../../spine-cpp/include")
        .clang_arg("-I../../src")
        .clang_arg("-fvisibility=default")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Link libraries - no C++ stdlib since we're using no-cpprt variant
    // The no-cpprt.cpp provides minimal runtime stubs

    println!("cargo:rerun-if-changed=../../spine-cpp/src");
    println!("cargo:rerun-if-changed=../../src");
    println!("cargo:rerun-if-changed=../../include/spine-c.h");
    println!("cargo:rerun-if-changed=wasm-include");
}
