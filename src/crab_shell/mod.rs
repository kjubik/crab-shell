mod handle_input;
use colored::Colorize;

pub fn run() {
    println!("Running Crab Shell...");

    let mut user_input = String::new();

    while user_input != "exit" {
        user_input.clear();
        user_input = handle_input::read_user_input();
        println!("User typed in '{}'", user_input.italic());
    }
}
