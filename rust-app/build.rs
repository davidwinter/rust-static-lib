fn main() {
    println!("cargo:rustc-link-search=native=../cpp-lib/build/lib");
    println!("cargo:rustc-link-lib=static=greeting");
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-flags=-l dylib=c++");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-flags=-l dylib=stdc++");
    }
}
