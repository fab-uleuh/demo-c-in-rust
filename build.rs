fn main() {
    println!("cargo:rerun-if-changed=src/c/example.c");
    println!("cargo:rerun-if-changed=src/c/example.h");

    cc::Build::new()
        .file("src/c/example.c")
        .include("src/c")
        .compile("example");
}
