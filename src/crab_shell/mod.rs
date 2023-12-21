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
            "help" => help(),
            _ => println!("Invalid command"),
        }
    }
}

fn help() {
    println!("
Welcome to Crab Shell!

Crab Shell is a Unix shell written in Rust, designed to provide a powerful and interactive command-line experience.

Available commands:

    help                        Display this help message
    exit                        Terminate the current shell session

Commands to be implemented:

    ls                          List directory contents
    cd <directory>              Change the current working directory
    pwd                         Print the current working directory
    echo <message>              Display a message to the console
    cat <file>                  Display the contents of a file
    touch <file>                Create an empty file
    rm <file>                   Remove a file
    mkdir <directory>           Create a new directory
    rmdir <directory>           Remove an empty directory
    cp <source> <dest>          Copy a file or directory
    mv <source> <dest>          Move or rename a file or directory
    chmod <permissions> <file>  Change the permissions of a file
    chown <user:group> <file>   Change the owner of a file
    head <file>                 Display the first few lines of a file
    tail <file>                 Display the last few lines of a file
    history                     Display command history
    alias <name> <command>      Create a shortcut for a command
    unalias <name>              Remove an alias
    wc <file>                   Count lines, words, and characters in a file
    date                        Display the current date and time
    cal                         Display a calendar
    whoami                      Display the current username

Useful Tips:
- Press TAB for auto-completion of commands and file paths.
- Use up and down arrow keys to navigate through command history.

For more information about a specific command, type 'help <command>'.
Feel free to explore and enjoy using Crab Shell!
");
}
