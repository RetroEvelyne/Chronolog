// User does something like chronolog log or chronolog read
// --log opens up the default editor with a markdown file template
// something like  1. [[Title]] 2. [[Date]]
// --read gives a list of files with the Title and Date showing, they can then select one to read
mod repo;
mod newlog;
mod readlog;

use clap::{command, Command, ArgMatches};

fn main() {
    let logs_repo_path = repo::logs_repo_string();

    match repo::ensure_logs_repo(&logs_repo_path) {
        Ok(..) => (),
        Err(error) => panic!("Error: {:?}", error)
    }

    let command_result = command!()
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
            .about("Log a new entry")
        )
        .subcommand(
            Command::new("edit")
            .about("Edit an existing entry")
        )
        .subcommand(
            Command::new("read")
            .about("Read an existing entry")
        )
        .get_matches();

    match command_result.subcommand() {
        Some(("new", subcommand_args)) => {
            let _ = newlog::new_log(subcommand_args, &logs_repo_path);
        },
        Some(("edit", subcommand_args)) => {
            edit_log(subcommand_args);
        },
        Some(("read", subcommand_args)) => {
            let _ = readlog::read_log(subcommand_args, &logs_repo_path);
        },
        Some((&_, _)) => {}
        None => {}
    }
}

fn edit_log(_args: &ArgMatches) {
}
