use colored::Colorize;
#[warn(unused_imports)]
use std::env;
//Pretty print users username
fn main() {
    if let Ok(username) = env::var("USER").or_else(|_| env::var("USERNAME")) {
        println!("You are running as user: {}", username.cyan().bold());

        return;
    }
}
