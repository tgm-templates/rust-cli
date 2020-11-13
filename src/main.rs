mod app;

use crate::app::{build_app, command_version, generate_shell_completion};

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    if matches.subcommand().is_none() {
        println!(
            "{}",
            "😂 Please use subcommand or --help to display help!"
        );
        return;
    }
    let (sub_command, args) = matches.subcommand().unwrap();
    if sub_command == "complete" {
        generate_shell_completion(args);
    } else if sub_command == "version" {
        println!("{}", command_version());
    } else {
        println!("sub command: {}", sub_command);
    }
}


