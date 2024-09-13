use std::io::{self, Write};

use colored::Colorize;
use commands::{CommandService, Context};
use controller::GoPro;

mod commands;
mod controller;

fn main() {
    let mut devices: Vec<GoPro> = Vec::new();
    devices.push(GoPro::new("test-device-1".into()));

    let mut test_device_2 = GoPro::new("test-device-2".into());
    test_device_2.recording = true;
    devices.push(test_device_2);

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
