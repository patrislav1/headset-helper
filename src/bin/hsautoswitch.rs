use clap::{App, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = App::new("Headset auto-switch helper")
        .about("Configures bluetooth headset auto-switching from A2DP to HSP/HFP")
        .version(VERSION)
        .args_from_usage(
            "-c, --config=[FILE] 'Set config file path'",
        )
        .subcommand(
            SubCommand::with_name("enable")
                .about("Enable auto-switch")
        )
        .subcommand(
            SubCommand::with_name("disable")
                .about("Disable auto-switch")
        )
        .get_matches();

    let cfg_file = args.value_of("config").unwrap_or("/etc/pulse/default.pa");
    println!("Value for config: {}", cfg_file);

    if let Some(cmd) = args.subcommand_name() {
        match cmd {
            "enable" => println!("Enable"),
            "disable" => println!("Disable"),
            _ => println!("Invalid command")
        }
    } else {
        println!("Just show setting")
    }
}
