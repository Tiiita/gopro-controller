use std::io::{self, Write};

use colored::Colorize;
use commands::Context;
use controller::GoPro;

mod commands;
mod controller;

fn main() {
    let mut devices: Vec<GoPro> = Vec::new();
    let test_device = GoPro::new("TestDevice".into());
    devices.push(test_device);
    println!();
    println!(
        "Welcome to the gopro controller shell, type {} for help!",
        "'help'".yellow()
    );
    init_shell(&devices);
}

pub fn init_shell(devices: &Vec<GoPro>) {
    loop {
        print!("=> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin line.");
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Bye.. :)");
            break;
        }

        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };

        let command_info = Context {
            name: command.into(),
            args: parts.collect(),
            devices: &devices,
        };

        match command {
            "help" => commands::help_cmd(command_info),
            "record" => commands::record_cmd(command_info),
            "devices" => commands::devices_cmd(command_info),
            _ => println!("{}", "Unkown command, type 'help' for help!".red()),
        }
    }
}
