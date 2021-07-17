use clap::{App, SubCommand};
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn extract_autoswitch_option(input: &str) -> Option<u8> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^\s*load-module\s+module-bluetooth-policy(?:\s+auto_switch=(?P<asw_option>\w+))?\s*$"
        ).unwrap();
    }
    RE.captures(input).and_then(|m| {
        match m.name("asw_option") {
            // If module-bluetooth-policy is given w/o auto_switch option, PulseAudio sets default to 1
            None => Some(1),
            Some(s) => match s.as_str().parse::<u8>() {
                Err(_) => None,
                Ok(n) => Some(n)
            }
        }
    })
}

fn read_autoswitch_option(fpath: &str) -> Option<u8> {
    match fs::read_to_string(fpath) {
        Ok(contents) => {
            for line in contents.lines() {
                if let Some(op) = extract_autoswitch_option(line) {
                    return Some(op);
                }
            }
            None
        },
        Err(e) => {
            println!("File error: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args = App::new("Headset auto-switch helper")
        .about("Configures headset bluetooth profile auto-switching")
        .version(VERSION)
        .args_from_usage(
            "-c, --config=[FILE] 'Set config file path'",
        )
        // see https://www.freedesktop.org/wiki/Software/PulseAudio/Documentation/User/Modules/#module-bluetooth-policy
        .subcommand(
            SubCommand::with_name("0")
                .about("Disable auto-switch")
        )
        .subcommand(
            SubCommand::with_name("1")
                .about("Enable auto-switch to HFP based on capture stream")
        )
        .subcommand(
            SubCommand::with_name("2")
                .about("Enable auto-switch to HFP based on heuristics")
        )
        .get_matches();

    let cfg_file = args.value_of("config").unwrap_or("/etc/pulse/default.pa");

    if let Some(cmd) = args.subcommand_name() {
        println!("{}", match cmd {
            "0" => "Disable",
            "1" => "HFP1",
            "2" => "HFP2",
            _ => "Invalid command"
        });
    } else {
        if let Some(i) = read_autoswitch_option(cfg_file) {
            println!("{}: Auto-switch to HFP {}", i, match i {
                0 => "disabled",
                1 => "enabled based on capture stream",
                2 => "enabled based on heuristics",
                _ => "N/A"
            })
        } else {
            println!("Couldn't parse config file");
            std::process::exit(1);
        }
    }
}
