mod crab_shell;
use colored::Colorize; // https://crates.io/crates/colored

fn main() {
    println!("{}\nA Unix shell written in Rust.", "Crab Shell".bold());
    println!("Type 'help' for more information.\n");

    crab_shell::run();
}
