use clap::ArgMatches;
use console::style;
use commands::{create, delete, rename, list};
use build_cli::build_cli;

mod build_cli;
mod commands;

pub fn fs() {
    let cli_matches = build_cli().get_matches();
    handle_command(&cli_matches);

        fn handle_command(cli_matches: &ArgMatches) {

            match cli_matches.subcommand() {
                
                Some(("create", sub_m)) => create::execute(sub_m),
                Some(("delete", sub_m)) => delete::execute(sub_m),
                Some(("rename", sub_m)) => rename::execute(sub_m),
                Some(("list", sub_m)) => list::execute(sub_m),
                
                Some((unknown, _)) => {
                    println!("{}", style(format!("Unknown command: {}", unknown)).yellow());
                }
                None => {
                    println!("{}", style("No command provided! Use --help to see available commands.").red());
                }
            }
        }
}
