use colored::Colorize;
use wifi_rs::WiFi;

use crate::gopro::GoPro;
pub type CommandResult<'a> = Result<(), CommandError<'a>>;

pub enum CommandError<'a> {
    Syntax,
    ExecutionFailed(&'a str),
}
pub struct CommandContext<'a> {
    pub name: String,
    pub args: Vec<&'a str>,
    pub devices: &'a mut Vec<GoPro>,
    pub cmd_service: &'a CommandService,
    pub connector: WiFi,
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    executor: Box<dyn Fn(CommandContext) -> CommandResult>,
}

impl Command {
    pub fn new<F>(
        name: &str,
        description: &str,
        usage: &str,
        executor: F,
    ) -> Self
    where for <'a>
        F: Fn(CommandContext) -> CommandResult + 'a,
    {
        Command {
            name: name.into(),
            usage: usage.into(),
            description: description.into(),
            executor: Box::new(executor),
        }
    }
    pub fn execute<'a>(&self, context: CommandContext<'a>) -> CommandResult<'a> {
        (self.executor)(context)
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

    pub fn execute(&self, context: CommandContext) {
        match self.find_by_name(&context.name) {
            Some(cmd) => {
                if let Err(error) = cmd.execute(context) {
                    match error {
                        CommandError::ExecutionFailed(msg) => println!("{}", msg.red()),
                        CommandError::Syntax => {
                            println!("{}", format!("Wrong syntax, use: {}", cmd.usage).red())
                        }
                    }
                }
            }
            None => println!(
                "{}",
                "Command not found, use 'help' to list all commands!".red()
            ),
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Command> {
        self.commands
            .iter()
            .find(|cmd| cmd.name.to_lowercase() == name.to_lowercase())
    }
}
