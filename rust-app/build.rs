fn main() {
    println!("cargo:rustc-link-search=native=../cpp-lib/build/lib");
    println!("cargo:rustc-link-lib=static=greeting");
}
