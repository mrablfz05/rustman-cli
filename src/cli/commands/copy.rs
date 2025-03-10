use clap::ArgMatches;
use console::style;
use std::fs;
use std::path::Path;

pub fn execute(sub_m: &ArgMatches) {
    let source = sub_m.get_one::<String>("source").unwrap();
    let destination = sub_m.get_one::<String>("destination").unwrap();

    if !Path::new(source).exists() {
        println!(
            "{}",
            style(format!("Error: Source file {} does not exist", source)).red()
        );
        return;
    }

    match fs::copy(source, destination) {
        Ok(_) => println!(
            "{}",
            style(format!("{} copied to {}", source, destination)).green()
        ),
        Err(e) => match e.kind() {
            std::io::ErrorKind::PermissionDenied => {
                eprintln!(
                    "{}",
                    style("Permission denied. Try running as admin!").red()
                );
            }
            std::io::ErrorKind::NotFound => {
                eprintln!("{}", style("File not found! Check the source path.").red());
            }
            std::io::ErrorKind::AlreadyExists => {
                eprintln!("{}", style("Destination file already exists.").yellow());
            }
            _ => {
                eprintln!("{}", style(format!("Copy failed: {}", e)).red());
            }
        },
    }
}
