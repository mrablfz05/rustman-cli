use clap::ArgMatches;
use console::style;
use std::fs;

pub fn execute(sub_m: &ArgMatches) {
    let filename = sub_m.get_one::<String>("filename").unwrap();

    match fs::read_to_string(filename) {
        Ok(contents) => {
            let splited_content: Vec<&str> = contents.split('.').collect();
            for splt_content in splited_content {
                if !splt_content.trim().is_empty() {
                    println!("{}", style(format!("{}", splt_content)).cyan())
                }
            }
        }
        Err(e) => println!(
            "{}",
            style(format!(
                "failed to show contents of {} because: {}",
                filename, e
            ))
            .red()
        ),
    }
}
