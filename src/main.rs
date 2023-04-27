use clap::{arg, Command};
use toml::{self, Table};

fn cli() -> Command {
    Command::new("cargo-workspace")
        .about("Cargo extention for working with workspaces")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .about("Install members of workspace")
                .arg(arg!(-f --force "Force overwriting existing crates or binaries")),
        )
        .subcommand(Command::new("uninstall").about("Uninstall members of workspace"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("install", _)) => {}
        Some(("uninstall", _)) => {}
        _ => {
            cli().print_help().unwrap();
            std::process::exit(1);
        }
    }

    let contents = match std::fs::read_to_string("Cargo.toml") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading Cargo.toml file:\n{}", e);
            std::process::exit(2);
        }
    };

    let data = match contents.parse::<Table>() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing Cargo.toml file:\n{}", e);
            std::process::exit(1);
        }
    };

    let workspace = match data.get("workspace") {
        Some(workspace) => workspace,
        None => {
            eprintln!("No workspace section found in Cargo.toml");
            std::process::exit(1);
        }
    };

    let members = match workspace.get("members") {
        Some(members) => members,
        None => {
            eprintln!("No members found in workspace section of Cargo.toml");
            std::process::exit(1);
        }
    };

    let members_vector = match members.as_array() {
        Some(members_vector) => members_vector,
        None => {
            eprintln!("Can not decode members as array, check your Cargo.toml file for errors");
            std::process::exit(1);
        }
    };

    for member in members_vector {
        let member_str = match member.as_str() {
            Some(member_str) => member_str,
            None => {
                eprintln!("Member is not a valid string, check your Cargo.toml file for errors");
                std::process::exit(1);
            }
        };

        match matches.subcommand() {
            Some(("install", subargs)) => {
                let force = subargs.get_flag("force");
                let mut args = vec!["install", "--path", member_str];
                if force {
                    args.push("--force");
                }
                cmd("cargo", &args);
            }
            Some(("uninstall", _)) => {
                cmd("cargo", &["uninstall", "-p", member_str]);
            }
            _ => {}
        }
    }
}

fn cmd(cmd: &str, args: &[&str]) {
    std::process::Command::new(cmd)
        .args(args)
        .status()
        .expect("Failed to execute command");
}
