use std::io;

use crate::{
    command::{CommandContext, CommandError, CommandResult},
    gopro::GoPro,
};

use colored::Colorize;
use futures::executor;
use tokio_wifiscanner::Wifi as WifiInfo;

pub fn help_cmd(context: CommandContext) -> CommandResult {
    let commands = &context.cmd_service.commands;

    let max_usage_len = commands
        .iter()
        .map(|command| command.usage.len())
        .max()
        .unwrap_or(0);

    println!();
    println!("----------- [{}] -----------", "HELP".yellow().bold());
    for command in commands {
        println!(
            "{:<width$} - {}",
            command.usage,
            command.description,
            width = max_usage_len
        );
    }

    println!();
    Ok(())
}

pub fn device_cmd(context: CommandContext) -> CommandResult {
    if context.args.is_empty() {
        return Err(CommandError::Syntax);
    }

    match context.args[0] {
        "list" => {
            if context.devices.is_empty() {
                return Err(CommandError::ExecutionFailed("No devices connected"));
            }

            println!("{:^15} | {:^10}", "Device Name", "Recording");
            println!("{:-<15}-+-{:-^15}", "", "");
            for gopro in context.devices {
                println!(
                    "{:^15} | {:^10}",
                    gopro.wifi_info.ssid,
                    if gopro.recording { "✅" } else { "❌" }
                );
            }
        }

        "add" => {
            let arg = context.args.get(1);

            if arg.is_none() {
                return Err(CommandError::Syntax);
            }

            let arg = arg.unwrap().to_lowercase();
            let access_points = scan();

            if !access_points
                .iter()
                .any(|gp| &gp.ssid.to_lowercase() == &arg)
            {
                return Err(CommandError::ExecutionFailed(
                    "Cannot find gopro with given name",
                ));
            }

            let wifi = access_points
                .into_iter()
                .find(|wifi| wifi.ssid.to_lowercase() == arg)
                .unwrap();
            let mut gopro = GoPro::new(wifi);

            let mut password = String::new();
            println!("Password for gopro '{arg}'");
            io::stdin()
                .read_line(&mut password)
                .expect("Failed to read line");

            match gopro.connect(password.as_str()) {
                Ok(_) => context.devices.push(gopro),
                Err(err) => {
                    eprintln!("{:?}", err);
                },
            }
        }

        "remove" => {
            println!("Unimplemented");
        }

        "scan" => {
            println!("Scanning, this may take some time..");

            let access_points = scan();

            if access_points.is_empty() {
                return Err(CommandError::ExecutionFailed(
                    "No nearby wifi access points found..",
                ));
            }
            println!(
                "{:^15} | {:^15} | {:^15}",
                "Device", "Strength", "Seems GoPro"
            );
            println!("{:^15}-+-{:^15}-+-{:^15}", "", "", "");
            for ele in access_points {
                println!(
                    "{:^15} | {:^15} | {:^15}",
                    ele.ssid,
                    ele.signal_level,
                    if ele.ssid.to_lowercase().starts_with("gp") {
                        "✅"
                    } else {
                        "❌"
                    }
                );
            }
        }
        _ => {
            return Err(CommandError::Syntax);
        }
    }

    Ok(())
}

pub fn record_cmd(_context: CommandContext) -> CommandResult {
    Ok(())
}

fn scan() -> Vec<WifiInfo> {
    executor::block_on(tokio_wifiscanner::scan()).expect("Failed to scan wifis")
}
