#[link(name = "greeting", kind = "static")]
extern "C" {
    fn say_greeting();
}

fn say_hello_from_cpp() {
    unsafe {
        say_greeting();
    }
}

fn main() {
    println!("Hello, world! from Rust");
    say_hello_from_cpp();
}
