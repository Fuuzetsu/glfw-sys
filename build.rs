extern crate cmake;
extern crate pkg_config;

use cmake::Config;

fn main() {
    if pkg_config::probe_library("glfw3").is_ok() {
        // We got the library via pkg_config, we don't need to try and build it
        // below.
        return;
    }

    let mut cfg = Config::new("glfw");

    cfg.define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib");

    let dst = if cfg!(feature = "wayland") {
        cfg.define("GLFW_USE_WAYLAND", "ON").build()
    } else {
        cfg.define("GLFW_USE_WAYLAND", "OFF").build()
    };

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=glfw3");
}
