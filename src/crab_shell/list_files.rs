use colored::Colorize;
use std::fs::{metadata, read_dir};

pub fn handle_listing_files(directory: &str) {
    match list_files(directory) {
        Ok(_) => {}
        Err(e) => eprintln!("Error listing files: {}", e),
    }
}

fn list_files(directory: &str) -> std::io::Result<()> {
    let entries = read_dir(directory)?;

    for entry in entries {
        let entry = entry?;
        let relative_path = entry.path();
        let shortened_path = relative_path.strip_prefix("./").unwrap_or(&relative_path);
        let metadata = metadata(&shortened_path)?;
        if metadata.is_dir() {
            println!("{}", shortened_path.display().to_string().blue());
        } else if metadata.is_file() {
            println!("{}", shortened_path.display().to_string());
        } else {
            println!("{}", shortened_path.display().to_string().yellow());
        }
    }

    Ok(())
}
