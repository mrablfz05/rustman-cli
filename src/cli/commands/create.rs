use clap::ArgMatches;
use console::style;
use std::fs;

pub fn execute(sub_m: &ArgMatches) {
    let filename = sub_m.get_one::<String>("filename").unwrap();
    if fs::File::create(filename).is_ok() {
        println!("{}", style(format!("File '{}' created", filename)).green());
    } else {
        println!(
            "{}",
            style(format!("Failed to create '{}'", filename)).red()
        );
    }
}
