use std::fs::read_dir;

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
        println!("{}", shortened_path.display());
    }

    Ok(())
}
