use colored::Colorize;

pub fn help_cmd(command: CommandInfo) {
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
pub fn devices_cmd(command: CommandInfo) {

}

pub fn record_cmd(command: CommandInfo) {
    
}

pub struct CommandInfo<'a> {
    pub name: String,
    pub args: Vec<&'a str>,
}