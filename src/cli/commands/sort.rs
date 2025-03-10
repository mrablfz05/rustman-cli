use clap::ArgMatches;
use console::style;
use std::fs;

pub fn execute(sub_m: &ArgMatches) {
    let filename = sub_m.get_one::<String>("filename").unwrap();
    let reverse = sub_m.get_flag("reverse");

    // Read the file contents
    match fs::read_to_string(filename) {
        Ok(content) => {
            // Split the content into lines and collect them into a vector
            let mut lines: Vec<&str> = content.lines().collect();

            // Sort the lines alphabetically
            lines.sort();

            // If reverse flag is set, reverse the order
            if reverse {
                lines.reverse();
            }

            // Join the sorted lines with newlines and print the result
            let sorted_content = lines.join("\n");
            println!("{}", sorted_content);
            println!("{}", style(format!("Sorted content:\n{}", sorted_content)).cyan());
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
        }
    }
}
