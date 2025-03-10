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
        Err(e) => println!("{}", style(format!("Copy failed: {}", e)).red()),
    }
}
