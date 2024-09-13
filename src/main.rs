use std::io::{self, Write};

use btleplug::platform::Adapter;
use colored::Colorize;
use futures::executor;
use goprosh::{
    command::{Command, CommandService, CommandContext},
    commands::{device_cmd, help_cmd, record_cmd},
    controller::GoPro,
};

#[tokio::main]
async fn main() {
    let mut devices: Vec<GoPro> = Vec::new();
    let mut central = executor::block_on(gopro_controller::init(None)).unwrap();

    println!();
    println!(
        "Welcome to the gopro controller shell, type {} for help!",
        "'help'".yellow()
    );
    init_shell(&mut devices, &mut central);
}

pub fn init_shell(devices: &mut Vec<GoPro>, gpc_central: &mut Adapter) {
    let mut cmd_service = CommandService::new();
    register_commands(&mut cmd_service);

    loop {
        print!("=> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin line.");
        let input = input.trim();

        let mut parts = input.split_whitespace();
        let cmd_name = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };

        let context = CommandContext {
            name: cmd_name.into(),
            args: parts.collect(),
            devices,
            cmd_service: &cmd_service,
            gpc_central,
        };

        cmd_service.execute(context);
    }
}

pub fn register_commands(service: &mut CommandService) {
    let commands = &mut service.commands;
    commands.push(Command::new(
        "exit",
        "Exits the program",
        "exit",
        |_context| {
            println!("Bye.. :)");
            std::process::exit(0);
        },
    ));

    commands.push(Command::new(
        "help",
        "List all commands and their usage",
        "help",
        help_cmd,
    ));

    commands.push(Command::new(
        "record",
        "Control record status of device(s)",
        "record <start, stop> <device | all>",
        record_cmd,
    ));

    commands.push(Command::new(
        "device",
        "Control and list the connected devices or scan for new ones",
        "device <list, add, remove, scan> <device | (all)>",
        device_cmd,
    ));
}
