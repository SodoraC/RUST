use std::io::{self, Write};

fn get_user_name() -> String {
    let mut name = String::new();

    print!("Please enter your name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

        name.trim().to_string()
}

fn main() {
    let name = get_user_name();
    println!("Hello, {}!", name);
}