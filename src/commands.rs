use colored::Colorize;

use crate::controller::GoPro;

pub fn help_cmd(context: Context) {
    let commands = get_commands();

    let max_usage_len = commands
        .iter()
        .map(|(usage, _)| usage.len())
        .max()
        .unwrap_or(0);

    println!();
    println!("----------- [{}] -----------", "HELP".yellow().bold());
    for (usage, description) in &commands {
        println!("{:<width$} - {}", usage, description, width = max_usage_len);
    }

    println!();
}
pub fn devices_cmd(context: Context) {
    if context.args.is_empty() {
        print_usage(&context.name);
        return;
    }
    for arg in context.args {
        let arg = arg.to_ascii_lowercase();

        match arg.as_str() {
            "list" => {
                if context.devices.is_empty() {
                    println!("No devices connected");
                    return;
                }

                println!(
                    "{:^15} | {:^15} | {:^10}",
                    "Device Name", "IP Address", "Recording"
                );
                println!("{:-<15}-+-{:-^15}-+-{:-<10}", "", "", "");
                for gopro in context.devices {
                    let recording_icon = if gopro.recording { "✅" } else { "❌" };
                    println!(
                        "{:^15} | {:^15} | {:^10}",
                        gopro.name, "unknown", recording_icon
                    );
                }
            }

            "add" => {
                println!("Unimplemented");
            }

            "remove" => {
                println!("Unimplemented");
            }

            "scan" => {
                println!("Unimplemented");
            }
            _ => {
               print_usage(&context.name);
            }
        }
    }
}

pub fn record_cmd(context: Context) {}

fn get_commands() -> Vec<(&'static str, &'static str)> {
    vec![
        ("help", "List all commands and their usage"),
        (
            "record <start, stop> <device | all>",
            "Control record status of device(s)",
        ),
        (
            "devices <list, add, remove, scan> <device | (all)>",
            "Control and list the connected devices or scan for new ones",
        ),
        (
            "exit",
            "Exit the program, all devices will hardly disconnect",
        ),
    ]
}

fn print_usage(command_name: &String) {
    if let Ok(usage) = get_usage(command_name) {
        let msg = format!("Wrong syntax, use: {}", get_usage(command_name).unwrap());
        println!("{}", msg.red());
    }
}
fn get_usage(command_name: &String) -> Result<String, String> {
    for command in get_commands() {
        if command.0.starts_with(command_name) {
            return Ok(command.0.into());
        }
    }

    return Err("No command found".into());
}
pub struct Context<'a> {
    pub name: String,
    pub args: Vec<&'a str>,
    pub devices: &'a Vec<GoPro>,
}
