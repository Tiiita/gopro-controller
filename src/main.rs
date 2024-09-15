use std::io::{self, Write};
use colored::Colorize;
use goprosh::{
    command::{Command, CommandContext, CommandService},
    commands::{device_cmd, help_cmd, record_cmd}, gopro::{self, GoPro},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut devices: Vec<GoPro> = Vec::new();
    println!();
    println!(
        "Welcome to the gopro controller shell, type {} for help!",
        "'help'".yellow()
    );
    init_shell(&mut devices);
}

pub fn init_shell(devices: &mut Vec<GoPro>) {
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
