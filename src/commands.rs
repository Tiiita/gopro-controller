use crate::command::{CommandContext, CommandError, CommandResult};

use colored::Colorize;
use futures::executor;
use gopro_controller as gpc;

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
            for _gopro in context.devices {
                let recording  = "unimplemented";

                let device_name = "unimplemented";
                //let recording_icon = if recording { "✅" } else { "❌" };
                println!("{:^15} | {:^10}", device_name, recording);
            }
        }

        "add" => {
            let arg = context.args.get(1);

            if arg.is_none() {
                return Err(CommandError::Syntax);
            }

            let arg = arg.unwrap();

            let gopros =
                executor::block_on(gopro_controller::scan(context.gpc_central)).unwrap();
            if !gopros.iter().any(|gp| &gp.to_lowercase().as_str() == arg) {
                return Err(CommandError::ExecutionFailed(
                    "Cannot find gopro with given name",
                ));
            }

            let mut central =
                executor::block_on(gpc::init(None)).expect("Unable to get adapter");
            let gopro = executor::block_on(gpc::connect(arg.to_string(), &mut central))
                .expect("Failed to connect");

            context.devices.push(gopro);
        }

        "remove" => {
            println!("Unimplemented");
        }

        "scan" => {
            println!("Scanning, this may take some time..");

            let gopros =
                executor::block_on(gopro_controller::scan(context.gpc_central)).unwrap();

            if gopros.is_empty() {
                return Err(CommandError::ExecutionFailed("No nearby gopros found.."));
            } else {
                println!("Found nearby gopros:");
                for ele in gopros {
                    println!("- {}", ele);
                }
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
