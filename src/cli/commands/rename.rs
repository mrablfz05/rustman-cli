use console::style;
use std::fs;
use clap::ArgMatches;

pub fn execute(sub_m: &ArgMatches) {
    let old_name = sub_m.get_one::<String>("oldname").unwrap();
    let new_name = sub_m.get_one::<String>("newname").unwrap();

    if fs::rename(old_name, new_name).is_ok() {
        println!("{}", style(format!("File {} renamed to {}", old_name, new_name)).green());
    } else {
        println!("{}", style(format!("Failed to rename {} to {}", old_name, new_name)).red());
    }
}