use std::io::{self, Write};

use btleplug::platform::Adapter;
use colored::Colorize;
use commands::{CommandService, Context};
use futures::executor;
use gopro_controller::GoPro;


mod commands;

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

pub fn init_shell(devices: &mut Vec<GoPro>, central: &mut Adapter) {
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
            devices,
            cmd_service: &cmd_service,
            gpl_central: central,
        };

        cmd_service.execute(context);
    }
}