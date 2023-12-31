mod handle_input;
mod list_files;

pub fn run() {
    let mut user_input = String::new();

    loop {
        user_input.clear();
        user_input = handle_input::read_user_input().to_string();
        match user_input.as_str() {
            "exit" => break,
            "help" => handle_displaying_help(),
            "ls" => list_files::handle_listing_files("."),
            _ => println!("Invalid command"),
        }
    }
}

fn handle_displaying_help() {
    println!("Welcome to Crab Shell!

Crab Shell is a Unix shell written in Rust, designed to provide a powerful and interactive command-line experience.

Available commands:

    help                        Display this help message
    exit                        Terminate the current shell session
    ls                          List directory contents

Feel free to explore and enjoy using Crab Shell!");
}
