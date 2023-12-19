use std::io::{self, Write};

pub fn read_user_input() -> String {
    let mut user_input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input = user_input.trim_end_matches('\n').to_string();

    return user_input;
}
