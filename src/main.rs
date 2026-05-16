use std::env;
fn main() {
    match env::home_dir() {
        Some(path) => println!("Your home dir is: {}", path.display()),
        None => println!("Impossible to get yout home dir")
    }
}