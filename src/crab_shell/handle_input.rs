use crossterm::event::{read, Event, KeyCode};
use crossterm::{cursor, execute};
use std::io::{self, stdout, Write};

pub fn read_user_input() -> String {
    let mut user_input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    // io::stdin()
    //     .read_line(&mut user_input)
    //     .expect("Failed to read line");
    // user_input = user_input.trim_end_matches('\n').to_string();

    handle_char_input(&mut user_input);

    return user_input;
}

fn handle_char_input(user_input: &mut String) {
    loop {
        match read().unwrap() {
            Event::Key(event) => match event.code {
                KeyCode::Left => {
                    execute!(stdout(), cursor::MoveLeft(1)).unwrap();
                }
                KeyCode::Right => {}
                KeyCode::Up => {}
                KeyCode::Down => {}
                KeyCode::Char(c) => {
                    print!("{}", c);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
