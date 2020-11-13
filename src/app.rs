//! clap App for command cli
use clap::{App, Arg};
use clap_generate::{generate, generators::Zsh};
use clap_generate::generators::Bash;
use std::env;
use std::fs::File;
use std::path::Path;
use clap::ArgMatches;

const COMMAND: &str = "@appName@";
const VERSION: &str = "0.1.0";

pub fn build_app() -> App<'static> {
    //todo add your sub commands here
    let version_command = App::new("version")
        .about("Show version details");
    // init Clap
    App::new(COMMAND)
        .version(VERSION)
        .about("command description here")
        .subcommand(version_command)
        .subcommand(complete_command())
}

pub fn generate_shell_completion(args: &ArgMatches) {
    let app = &mut build_app();
    if args.is_present("zsh") {
        generate::<Zsh, _>(app, COMMAND, &mut std::io::stdout());
    } else if args.is_present("bash") {
        generate::<Bash, _>(app, COMMAND, &mut std::io::stdout());
    } else if args.is_present("oh_my_zsh") {
        let home = env::var("HOME").unwrap();
        let dest_dir = format!("{}/.oh-my-zsh/custom/plugins/{}", home, COMMAND);
        let result = std::fs::create_dir_all(Path::new(&dest_dir));
        if result.is_ok() {
            // write _tgm file to plugin directory
            let dest_file = format!("{}/_{}", dest_dir, COMMAND);
            let mut file = File::create(dest_file).unwrap();
            generate::<Zsh, _>(app, COMMAND, &mut file);
            println!("💯 {} for oh-my-zsh generated successfully! Please add '{}' in plugins of ~/.zshrc file.", COMMAND, COMMAND)
        } else {
            println!("Failed to create directory: {}", &dest_dir);
        }
    }
}

pub fn command_version() -> String {
    return format!("Version: {}", VERSION);
}

fn complete_command() -> App<'static> {
    App::new("complete")
        .about("Generate shell completion for zsh & bash")
        .arg(
            Arg::new("zsh")
                .long("zsh")
                .takes_value(false)
                .about("Zsh completion")
                .required(false),
        )
        .arg(
            Arg::new("oh_my_zsh")
                .long("oh_my_zsh")
                .takes_value(false)
                .about("Oh My Zsh")
                .required(false),
        )
        .arg(
            Arg::new("bash")
                .long("bash")
                .takes_value(false)
                .about("Bash completion")
                .required(false),
        )
}
