fn main() {
    println!("cargo:rustc-link-search=native=../cpp-lib/build");
    println!("cargo:rustc-link-lib=greeting");
}
