use std::env;
use std::fs;
use std::path::PathBuf;

#[path = "build/encode.rs"]
mod encode;
#[path = "build/cat.rs"]
mod cat;
#[path = "build/themes.rs"]
mod themes;
#[path = "build/props.rs"]
mod props;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy("memory.x", out.join("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build/encode.rs");
    println!("cargo:rerun-if-changed=build/cat.rs");
    println!("cargo:rerun-if-changed=build/themes.rs");
    println!("cargo:rerun-if-changed=build/props.rs");

    let mut src = String::new();
    cat::emit(&mut src, &out);
    themes::emit(&mut src, &out);
    props::emit(&mut src, &out);

    fs::write(out.join("sprites.rs"), src).unwrap();
}
