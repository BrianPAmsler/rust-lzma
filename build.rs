use std::env;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let is_windows = target.contains("windows");
    let statik = env::var("CARGO_FEATURE_STATIC").is_ok();

    if is_windows {
        // Install vcpkg
        Command::new("cargo").args(&["install", "cargo-vcpkg"]).status().unwrap();
        Command::new("cargo").args(&["vcpkg", "build"]).status().unwrap();
        
        #[cfg(windows)]
        vcpkg::Config::new()
            .emit_includes(true)
            .find_package("liblzma")
            .expect("Could not find liblzma using vcpkg");
    } else {
        #[cfg(not(windows))]
        pkg_config::Config::new()
            .statik(statik)
            .probe("liblzma")
            .expect("Could not find liblzma using pkg-config");
    }

    if statik {
        println!("cargo:rustc-link-lib=static=lzma");
    }
}
