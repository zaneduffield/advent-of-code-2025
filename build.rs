use std::path::Path;

fn main() {
    if Path::new("input").is_dir() {
        println!("cargo:rustc-cfg=input_exists");
    }
    println!("cargo::rustc-check-cfg=cfg(input_exists)");
}
