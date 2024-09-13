use std::process;

use colored::Colorize;

use crate::controller::{self, GoPro};

fn help_cmd(context: Context) -> Result<(), CommandError>{
    let commands = &context.cmd_service.commands;

    let max_usage_len = commands
        .iter()
        .map(|command| command.usage.len())
        .max()
        .unwrap_or(0);

    println!();
    println!("----------- [{}] -----------", "HELP".yellow().bold());
    for command in commands {
        println!("{:<width$} - {}", command.usage, command.description, width = max_usage_len);
    }

    println!();
    Ok(())
}

fn devices_cmd(context: Context) -> Result<(), CommandError> {
    if context.args.is_empty() {
        return Err(CommandError::Syntax);
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    for arg in context.args {
        let arg = arg.to_ascii_lowercase();

        match arg.as_str() {
            "list" => {
                if context.devices.is_empty() {
                    return Err(CommandError::ExecutionFailed("No devices connected"));
                }

                println!(
                    "{:^15} | {:^10}",
                    "Device Name", "Recording"
                );
                println!("{:-<15}-+-{:-^15}-+-{:-<10}", "", "", "");
                for gopro in context.devices {
                    let recording_icon = if gopro.recording { "✅" } else { "❌" };
                    println!(
                        "{:^15} | {:^10}",
                        gopro.name, recording_icon
                    );
                }
            }

            "add" => {
            }

            "remove" => {
                println!("Unimplemented");
            }

            "scan" => {
                println!("Scanning, this may take some time..");
            
                let gopros = rt.block_on(controller::discover());
                if gopros.is_empty() { return Err(CommandError::ExecutionFailed("No nearby gopros found..")) } else {
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

    }
    Ok(())
}

fn record_cmd(_context: Context) -> Result<(), CommandError> {
    Ok(())
}


enum CommandError<'a> {
    Syntax,
    ExecutionFailed(&'a str),
}
pub struct Context<'a> {
    pub name: String,
    pub args: Vec<&'a str>,
    pub devices: &'a Vec<GoPro>,
    pub cmd_service: &'a CommandService,
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    executor: Box<dyn Fn(Context) -> Result<(), CommandError>>,
}

impl Command {
     fn new(name: &str, description: &str, usage: &str, executor: Box<dyn Fn(Context) -> Result<(), CommandError>>) -> Self {
        Command {
            name: name.into(),
            usage: usage.into(),
            description: description.into(),
            executor,
        }
    }
}
pub struct CommandService {
    pub commands: Vec<Command>,
}
impl CommandService {
    pub fn new() -> Self {
        CommandService {
            commands: Vec::new(),
        }
    }

    pub fn execute(&self, context: Context) {
        match self.find_by_name(&context.name) {
            Some(cmd) => {

               if let Err(error) = (cmd.executor)(context) {
                match error {
                    CommandError::ExecutionFailed(msg) => println!("{}", msg.red()),
                    CommandError::Syntax => println!("{}", format!("Wrong syntax, use: {}", cmd.usage).red()),
                }
               }

            },
            None => println!("{}", "Command not found, use 'help' to list all commands!".red()),
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Command> {
        self.commands
            .iter()
            .find(|cmd| cmd.name.to_lowercase() == name.to_lowercase())
    }
}

pub fn register_commands(service: &mut CommandService) {
    let commands = &mut service.commands;
    commands.push(Command::new(
        "exit",
        "Exits the program",
        "exit",
        Box::new(|_context| {
            println!("Bye.. :)");
            process::exit(0);
        }),
    ));

    commands.push(Command::new(
        "help",
        "List all commands and their usage",
        "help",
        Box::new(|context| help_cmd(context)),
    ));

    commands.push(Command::new(
        "record",
        "Control record status of device(s)",
        "record <start, stop> <device | all>",
        Box::new(|context| record_cmd(context)),
    ));

    commands.push(Command::new(
        "devices",
        "Control and list the connected devices or scan for new ones",
        "devices <list, add, remove, scan> <device | (all)>",
        Box::new(|context| devices_cmd(context)),
    ));
}
