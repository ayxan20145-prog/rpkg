use std::env;

fn main() {
    match env::args().nth(1).as_deref() {
        Some("install") => println!("No packages yet :("),
        Some(cmd) => println!("Unknown command: {}", cmd),
        None => println!("Usage: \ntarman install <package>"),
    }
}
