#![feature(cfg_target_feature)]

use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut lib_dir = manifest_dir.clone();
    lib_dir.push("lib");

    if target.contains("pc-windows") {
        if target.contains("msvc") {
            lib_dir.push("msvc");
        } else {
            lib_dir.push("mingw");
        }
    } else if target.contains("linux") {
        lib_dir.push("linux");
    } else if target.contains("darwin") {
        lib_dir.push("darwin");
    } else {
        return;
    };

    if target.contains("x86_64") {
        lib_dir.push("64");
    } else {
        lib_dir.push("32");
    }

    if target.contains("x86_64-pc-windows-msvc") {
        println!("cargo:rustc-link-lib=fmod64_vc");
        println!("cargo:rustc-link-lib=fmodstudio64_vc");
    } else {
        println!("cargo:rustc-link-lib=fmod");
        println!("cargo:rustc-link-lib=fmodstudio");
    }

    println!("cargo:rustc-link-search=all={}", lib_dir.display());
    if let Ok(read_dir) = std::fs::read_dir(lib_dir) {
        for entry in read_dir {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path_0 = manifest_dir.clone();
            new_file_path_0.push("target");
            new_file_path_0.push(env::var("PROFILE").unwrap());
            let mut new_file_path_1 = PathBuf::from(env::var("OUT_DIR").unwrap());

            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") || file_name.ends_with(".dylib") ||
                    file_name.contains(".so")
                {
                    new_file_path_0.push(&file_name);
                    new_file_path_1.push(&file_name);
                    std::fs::copy(&entry_path, new_file_path_0.as_path())
                        .map_err(|_| eprintln!("Can't copy from DLL dir"))
                        .ok();
                    std::fs::copy(&entry_path, new_file_path_1.as_path())
                        .map_err(|_| eprintln!("Can't copy from DLL dir"))
                        .ok();
                }
            }
        }
    }
}
