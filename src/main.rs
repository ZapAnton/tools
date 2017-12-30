//! Amethyst CLI binary crate.
//!

extern crate amethyst_cli;
extern crate ansi_term;
extern crate clap;

use std::process::exit;

use amethyst_cli as cli;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

fn main() {
    let matches = App::new("Amethyst CLI")
        .author("Created by Amethyst developers")
        .version("1.0.2")
        .about("Allows managing Amethyst game projects")
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new Amethyst project")
                .arg(
                    Arg::with_name("project_name")
                        .help("The directory name for the new project")
                        .required(true),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        ("new", Some(args)) => exec_new(args),
        _ => eprintln!("WARNING: subcommand not tested. This is a bug."),
    }
}

fn exec_new(args: &ArgMatches) {
    let project_name = args.value_of("project_name")
        .expect("Bug: project_name is required");
    let project_name = project_name.to_owned();

    let n = cli::New {
        project_name,
        ..Default::default()
    };

    if let Err(e) = n.execute() {
        handle_error(e);
    }
}

fn handle_error(e: cli::error::Error) {
    use ansi_term::Color;

    eprintln!("{}: {}", Color::Red.paint("error"), e);

    e.iter()
        .skip(1)
        .for_each(|e| eprintln!("{}: {}", Color::Red.paint("caused by"), e));

    // Only shown if `RUST_BACKTRACE=1`.
    if let Some(backtrace) = e.backtrace() {
        eprintln!();
        eprintln!("backtrace: {:?}", backtrace);
    }

    exit(1);
}