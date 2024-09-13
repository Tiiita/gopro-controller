use colored::Colorize;
use futures::executor;
use gopro_controller as gp_ctrl;
use crate::{command::{CommandError, CommandResult, CommandContext}, controller::{self, GoPro}};

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

pub fn device_cmd<'a>(context: CommandContext<'a>) -> CommandResult<'a> {
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
                let recording_icon = if gopro.recording { "✅" } else { "❌" };
                println!("{:^15} | {:^10}", gopro.name, recording_icon);
            }
        }

        "add" => {
            let arg = context.args.get(1);

            if arg.is_none() {
                return Err(CommandError::Syntax);
            }

            let arg = arg.unwrap();
    
            let gopros = executor::block_on(controller::discover());
            if !gopros
                .iter()
                .any(|gp| &gp.name.to_lowercase().as_str() == arg)
            {
                return Err(CommandError::ExecutionFailed(
                    "Cannot find gopro with given name",
                ));
            }

            let mut central = executor::block_on(gp_ctrl::init(None)).expect("Unable to get adapter");
            executor::block_on(gp_ctrl::connect(arg.to_string(), &mut central)).expect("Failed to connect");

            context.devices.push(GoPro::new(arg.to_string()));
        }

        "remove" => {
            println!("Unimplemented");
        }

        "scan" => {
            println!("Scanning, this may take some time..");

            let gopros = executor::block_on(controller::discover());
            if gopros.is_empty() {
                return Err(CommandError::ExecutionFailed("No nearby gopros found.."));
            } else {
                println!("Found nearby gopros:");
                for ele in gopros {
                    println!("- {}", ele.name);
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