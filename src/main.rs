use std::io::{self, Write};

use colored::Colorize;
use commands::CommandInfo;

mod commands;
mod controller;

fn main() {
    println!();
    println!(
        "Welcome to the gopro controller shell, type {} for help!",
        "'help'".yellow()
    );
    init_shell();
}

pub fn init_shell() {
    loop {
        print!("=> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin line.");
        let input = input.trim();
        if (input.eq_ignore_ascii_case("exit")) {
            println!("Bye.. :)");
            break;
        }

        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };

        let command_info = CommandInfo {
            name: command.into(),
            args: parts.collect(),
        };

        match command {
            "help" => commands::help_cmd(command_info),
            "record" => commands::record_cmd(command_info),
            "devices" => commands::devices_cmd(command_info),
            _ => println!("{}", "Unkown command, type 'help' for help!".red()),
        }
    }
}
