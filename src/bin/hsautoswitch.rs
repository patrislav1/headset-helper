use std::{fs, io};
use std::io::Write;
use clap::{App, SubCommand};
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

fn read_autoswitch_option(fpath: &str) -> io::Result<u8> {
    let contents = fs::read_to_string(fpath)?;
    for line in contents.lines() {
        if let Some(op) = extract_autoswitch_option(line) {
            return Ok(op);
        }
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "Parse error"))
}

fn write_autoswitch_option(fpath: &str, op: u8) -> io::Result<()> {
    let contents = fs::read_to_string(fpath)?;
    let mut file = fs::File::create(fpath)?;
    for line in contents.lines() {
        if let Some(_) = extract_autoswitch_option(line) {
            file.write_fmt(
                format_args!("load-module module-bluetooth-policy auto_switch={}\n", op)
            )?;
        } else {
            file.write_fmt(
                format_args!("{}\n", line)
            )?;
        }
    }
    Ok(())
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
        if let Err(why) = write_autoswitch_option(cfg_file, cmd.parse::<u8>().unwrap()) {
            println!("couldn't write {}: {}", cfg_file, why);
            std::process::exit(1);
        }
    } else {
        match read_autoswitch_option(cfg_file) {
            Err(why) => {
                println!("couldn't read {}: {}", cfg_file, why);
                std::process::exit(1);
            },
            Ok(i) => println!("{}: Auto-switch to HFP {}", i, match i {
                0 => "disabled",
                1 => "enabled based on capture stream",
                2 => "enabled based on heuristics",
                _ => "N/A"
            })
        }
    }
}
