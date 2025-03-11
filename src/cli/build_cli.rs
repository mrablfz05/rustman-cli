use clap::{Arg, ArgAction, Command};

pub fn build_cli() -> Command {
    Command::new("RustMan")
        .version("1.0")
        .author("mrablfz")
        .about("Rust file-based CLI tool")
        .subcommand(create_command())
        .subcommand(delete_command())
        .subcommand(rename_command())
        .subcommand(list_command())
        .subcommand(copy_command())
        .subcommand(sort_command())
        .subcommand(curl_command())
        .subcommand(mv_command())
        .subcommand(find_command())
}

fn create_command() -> Command {
    Command::new("create").about("Create a file").arg(
        Arg::new("filename")
            .help("Name of the file to create")
            .required(true)
            .index(1),
    )
}

fn delete_command() -> Command {
    Command::new("delete").about("Delete a file").arg(
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

fn copy_command() -> Command {
    Command::new("copy")
        .about("Copy a file to a destination")
        .arg(Arg::new("source").help("Copy file").required(true).index(1))
        .arg(
            Arg::new("destination")
                .help("Pase to destination")
                .required(true)
                .index(2),
        )
}

fn sort_command() -> Command {
    Command::new("sort")
        .about("Sort file")
        .arg(
            Arg::new("filename")
                .help("Sort ascending file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .help("Sort in reverse order")
                .action(ArgAction::SetTrue),
        )
}

fn curl_command() -> Command {
    Command::new("curl").about("Fetch data from url").arg(
        Arg::new("url")
            .help("Fetch data from a url")
            .required(true)
            .index(1),
    )
}

pub fn mv_command() -> Command {
    Command::new("mv")
        .about("Move a file to a destination")
        .arg(
            Arg::new("source")
                .help("The source file to move")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("destination")
                .help("The destination path")
                .required(true)
                .index(2),
        )
}

fn find_command() -> Command {
    Command::new("find")
        .about("Search for a file by name in a directory")
        .arg(Arg::new("directory").help("Directory to search in").required(true).index(1))
        .arg(Arg::new("filename").help("Filename to search for").required(true).index(2))
}