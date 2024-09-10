
use colored::Colorize;

use crate::controller::GoPro;

pub fn help_cmd(context: Context) {
    let commands = vec![
        ("help", "List all commands and their usage"),
        ("record <start, stop> <device | all>", "Control record status of device(s)"),
        ("devices <list, add, remove, scan> <device | (all)>", "Control and list the connected devices or scan for new ones"),
        ("exit", "Exit the program, all devices will hardly disconnect"),
    ];
    
    let max_usage_len = commands.iter().map(|(usage, _)| usage.len()).max().unwrap_or(0);

    println!();
    println!("----------- [{}] -----------", "HELP".yellow().bold());
    for (usage, description) in &commands {
        println!("{:<width$} - {}", usage, description, width = max_usage_len);
    }

    println!();
}
pub fn devices_cmd(context: Context) {
    for arg in context.args {
        let arg  = arg.to_ascii_lowercase();

        match arg.as_str() {
            "list" => {
                if context.devices.is_empty() {
                    println!("No devices connected");
                    return;
                }

                println!("{:^15} | {:^15} | {:^10}", "Device Name", "IP Address", "Recording");
                for gopro in context.devices {
                    let recording_icon = if gopro.recording {"✅"} else {"❌"};
                    println!("{:^15} | {:^15} | {:^10}", gopro.name, "unknown", recording_icon);
                }
            }
            _ => { 
                if context.args.is_empty() {

                }
                unknown_arg(arg);
            }
        }
    }
}

pub fn record_cmd(context: Context) {
    
}

fn unknown_arg(arg: String) {
    println!("{}", "Unknown argument: '{}', type 'help'".red().to_string().replace("{}", &arg));
}

pub struct Context<'a> {
    pub name: String,
    pub args: Vec<&'a str>,
    pub devices: &'a Vec<GoPro>,
}