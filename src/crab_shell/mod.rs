mod handle_input;
use colored::Colorize;

pub fn run() {
    let mut user_input = String::new();

    loop {
        user_input.clear();
        user_input = handle_input::read_user_input().to_string();
        // println!("User typed in '{}'", user_input.italic());
        match user_input.as_str() {
            "exit" => break,
            "help" => handle_displaying_help(),
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
