use clap::{Arg, Command};
use console::style;
use std::fs;

pub fn fs() {
    let cli_matches = Command::new("RustMan")
        .version("1.0")
        .author("mrablfz")
        .about("Rust file-based CLI tool")
        .subcommand(
            Command::new("create")
                .about("Create a file")
                .arg(
                    Arg::new("filename")
                        .help("Name of the file to create")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a file")
                .arg(
                    Arg::new("filename")
                        .help("Name of the file to delete")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("rename")
                .about("Rename a file")
                .arg(
                    Arg::new("oldname")
                        .help("Old file name")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("newname")
                        .help("New file name")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(Command::new("list").about("List directory files"))
        .get_matches();

    match cli_matches.subcommand() {
        // create command
        Some(("create", sub_m)) => {
            let filename = sub_m.get_one::<String>("filename").unwrap();
            if fs::File::create(filename).is_ok() {
                println!("{}", style(format!("File '{}' created", filename)).green());
            } else {
                println!("{}", style(format!("Failed to create '{}'", filename)).red());
            }
        }
        
        // delete command
        Some(("delete", sub_m)) => {
            let filename = sub_m.get_one::<String>("filename").unwrap();
            if fs::remove_file(filename).is_ok() {
                println!("{}", style(format!("File {} deleted!", filename)).green());
            } else {
                println!("{}", style(format!("Failed to delete {}", filename)).red());
            }
        }

        // rename command
        Some(("rename", sub_m)) => {
            let old_name = sub_m.get_one::<String>("oldname").unwrap();
            let new_name = sub_m.get_one::<String>("newname").unwrap();

            if fs::rename(old_name, new_name).is_ok() {
                println!("{}", style(format!("File {} renamed to {}", old_name, new_name)).green());
            } else {
                println!("{}", style(format!("Failed to rename {} to {}", old_name, new_name)).red());
            }
        }

        // list command
        Some(("list", sub_m)) => {
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
                    println!("{}", style(format!("Failed to list directory '{}'", path)).red());
                }
            }
        }

        Some((unknown, _)) => {
            println!("{}", style(format!("Unknown command: {}", unknown)).yellow());
        }
        None => {
            println!("{}", style("No command provided! Use --help to see available commands.").red());
        }
    }
}
