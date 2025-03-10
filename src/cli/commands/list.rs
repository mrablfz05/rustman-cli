use clap::ArgMatches;
use console::style;
use std::fs;

pub fn execute(sub_m: &ArgMatches) {
    let default_path = &".".to_string();
    let path = sub_m.get_one::<String>("path").unwrap_or(default_path);
    match fs::read_dir(path) {
        Ok(entries) => {
            println!("{}", style(format!("Listing files in '{}':", path)).cyan());
            for entry in entries.flatten() {
                println!("{}", style(entry.file_name().to_string_lossy()).yellow());
            }
        }
        Err(_) => {
            println!(
                "{}",
                style(format!("Failed to list directory '{}'", path)).red()
            );
        }
    }
}
