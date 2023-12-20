use crossterm::event::{read, Event, KeyCode};
use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, stdout, Write};

pub fn read_user_input() -> String {
    let mut user_input = String::new();

    print!("$ ");
    io::stdout().flush().unwrap();
    handle_char_input(&mut user_input);

    return user_input;
}

fn handle_char_input(user_input: &mut String) {
    enable_raw_mode().unwrap();

    let mut index = 0;

    loop {
        match read().unwrap() {
            Event::Key(event) => match event.code {
                KeyCode::Enter => {
                    print!("\r\n");
                    break;
                }
                KeyCode::Backspace => {
                    if index == 0 {
                        continue;
                    };
                    // logic
                    index -= 1;
                    user_input.remove(index);
                    // display
                    execute!(stdout(), cursor::MoveLeft(1)).unwrap();
                    print!(" ");
                    execute!(stdout(), cursor::MoveLeft(1)).unwrap();

                    execute!(stdout(), cursor::SavePosition).unwrap();
                    execute!(stdout(), Clear(ClearType::UntilNewLine)).unwrap();
                    for c in user_input.chars().skip(index) {
                        print!("{}", c);
                    }
                    io::stdout().flush().unwrap();
                    execute!(stdout(), cursor::RestorePosition).unwrap();
                }
                KeyCode::Left => {
                    if index == 0 {
                        continue;
                    }
                    // logic
                    index -= 1;
                    // display
                    execute!(stdout(), cursor::MoveLeft(1)).unwrap();
                }
                KeyCode::Right => {
                    if index == user_input.len() {
                        continue;
                    }
                    // logic
                    index += 1;
                    // display
                    execute!(stdout(), cursor::MoveRight(1)).unwrap();
                }
                KeyCode::Up => {}
                KeyCode::Down => {}
                KeyCode::Char(c) => {
                    if index == user_input.len() {
                        user_input.push(c);
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                    }
                    else {
                        user_input.insert(index, c);
                        execute!(stdout(), cursor::SavePosition).unwrap();
                        execute!(stdout(), Clear(ClearType::UntilNewLine)).unwrap();
                        for c in user_input.chars().skip(index) {
                            print!("{}", c);
                        }
                        io::stdout().flush().unwrap();
                        execute!(stdout(), cursor::RestorePosition).unwrap();
                        execute!(stdout(), cursor::MoveRight(1)).unwrap();
                    }

                    index += 1;
                }
                _ => {}
            },
            _ => {}
        }
    }

    disable_raw_mode().unwrap();
}
