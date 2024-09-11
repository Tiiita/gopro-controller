use core::str;
use std::io::{self, Write};

use colored::Colorize;
use commands::{CommandService, Context};
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
    let mut cmd_service = CommandService::new();
    commands::register_commands(&mut cmd_service);

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
        let cmd_name = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };

        let context = Context {
            name: cmd_name.into(),
            args: parts.collect(),
            devices: &devices,
            cmd_service: &cmd_service,
        };

        cmd_service.execute(context);
    }
}
