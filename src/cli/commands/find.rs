use clap::ArgMatches;
// use std::path;
use console::style;
use std::fs;

pub fn execute(sub_m: &ArgMatches) {

    let dir = sub_m.get_one::<String>("directory").unwrap();
    let filename = sub_m.get_one::<String>("filename").unwrap();

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if entry.file_name().to_string_lossy().contains(filename) {
                    println!("{}", style(entry.path().display()).green());
                }
            }
        }
        Err(e) => eprintln!("{}", style(format!("Error: {}", e)).red()),
    }
}