use std::path::Path;
use std::process::Command;

fn main() {
    Command::new("make")
        .args(&["clean"])
        .current_dir(&Path::new("./native"))
        .status()
        .unwrap();

    Command::new("make")
        .args(&["static"])
        .current_dir(&Path::new("./native"))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", "./target/native");
    println!("cargo:rustc-link-lib=static=tide-trace");
}
