use clap::ArgMatches;
use console::style;
use std::fs;
use std::io::ErrorKind;

pub fn execute(sub_m: &ArgMatches) {
    let source = sub_m.get_one::<String>("source").unwrap();
    let destination = sub_m.get_one::<String>("destination").unwrap();

    match fs::rename(source, destination) {
        Ok(_) => println!("{}", style(format!("✅ File {} moved to {}", source, destination)).green()),
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => eprintln!("{}", style("❌ Permission denied! Try running as admin.").red()),
            ErrorKind::NotFound => eprintln!("{}", style("❌ Source file not found!").red()),
            ErrorKind::AlreadyExists => eprintln!("{}", style("⚠️ Destination file already exists! Choose a different name.").yellow()),
            _ => eprintln!("{}", style(format!("❌ Failed to move file: {} | More info: {}", source, e)).red()),
        },
    }
}
