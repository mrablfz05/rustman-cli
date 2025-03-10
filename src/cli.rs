use build_cli::build_cli;
use clap::ArgMatches;
use commands::{copy, create, delete, list, rename};
use console::style;

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
            Some(("copy", sub_m)) => copy::execute(sub_m),

            Some((unknown, _)) => {
                println!(
                    "{}",
                    style(format!("unknown command: {}", unknown)).yellow()
                );
            }
            None => {
                println!(
                    "{}",
                    style("No command provided! Use --help to see available commands.").red()
                );
            }
        }
    }
}
