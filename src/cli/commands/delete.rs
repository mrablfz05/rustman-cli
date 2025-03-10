use console::style;
use std::fs;
use clap::ArgMatches;

pub fn execute(sub_m: &ArgMatches) {
    let filename = sub_m.get_one::<String>("filename").unwrap();
    if fs::remove_file(filename).is_ok() {
        println!("{}", style(format!("File {} deleted!", filename)).green());
    } else {
        println!("{}", style(format!("Failed to delete {}", filename)).red());
    }
}