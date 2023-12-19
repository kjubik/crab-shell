use crossterm::event::{read, Event, KeyCode};
use crossterm::{cursor, execute, terminal::{enable_raw_mode, disable_raw_mode}};
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
    enable_raw_mode().unwrap();

    loop {
        match read().unwrap() {
            Event::Key(event) => match event.code {
                KeyCode::Enter => break,
                KeyCode::Left => {
                    execute!(stdout(), cursor::MoveLeft(1)).unwrap();
                }
                KeyCode::Right => {}
                KeyCode::Up => {}
                KeyCode::Down => {}
                KeyCode::Char(c) => {
                    user_input.push(c);
                    print!("{}", c);
                    io::stdout().flush().unwrap();
                }
                _ => {}
            },
            _ => {}
        }
    }

    disable_raw_mode().unwrap();
}
