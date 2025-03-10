use clap::ArgMatches;
use console::style;
use std::fs;

pub fn execute(sub_m: &ArgMatches) {
    let filename = sub_m.get_one::<String>("filename").unwrap();
    let reverse = sub_m.get_flag("reverse");

    match fs::read_to_string(filename) {
        Ok(content) => {
            let mut lines: Vec<&str> = content.split_whitespace().collect();

            lines.sort();

            if reverse {
                lines.reverse();
            }

            let sorted_content = lines.join("\n");
            println!("{}", style(format!("Sorted content:\n{}", sorted_content)).cyan());
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
        }
    }
}
