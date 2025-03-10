use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("RustMan")
        .version("1.0")
        .author("mrablfz")
        .about("Rust file-based CLI tool")
        .subcommand(create_command())
        .subcommand(delete_command())
        .subcommand(rename_command())
        .subcommand(list_command())
}

fn create_command() -> Command {
            Command::new("create")
                .about("Create a file")
                .arg(
                    Arg::new("filename")
                        .help("Name of the file to create")
                        .required(true)
                        .index(1),
                )
}

fn delete_command() -> Command {
    Command::new("delete")
                .about("Delete a file")
                .arg(
                    Arg::new("filename")
                        .help("Name of the file to delete")
                        .required(true)
                        .index(1),
                )
        
}

fn rename_command() -> Command {
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
                )
}
fn list_command() -> Command {
    Command::new("list").about("List directory files")
}