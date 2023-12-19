mod user_input;
use std::io::{self, Write};
use colored::Colorize;

pub fn run() {
    println!("Running Crab Shell...");

    let mut user_input = String::new();

    while user_input.trim() != "exit" {
        user_input.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input = user_input.trim_end().to_string();
        println!("user typed in '{}'", user_input.italic());
    }
}
