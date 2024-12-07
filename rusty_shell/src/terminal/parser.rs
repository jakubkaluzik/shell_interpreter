pub enum Command {
    Cd(String),
    Ls,
    Clear,
    Ok,
    Err(Vec<String>),
}
pub fn parse(dir: &String, input: &String) -> Command {
    let mut parts = input.split_whitespace();
    match parts.next() {
        Some("cd") => {
            if let Some(target) = parts.next() {
                if parts.next().is_none() {
                    Command::Cd(target.to_string())
                } else {
                    Command::Err(vec!["Too many arguments for 'cd' command.".to_string()])
                }
            } else {
                Command::Err(vec!["No target directory specified for 'cd' command.".to_string()])
            }
        }
        Some("ls") => Command::Ls,
        Some("clear") => Command::Clear,
        _ => Command::Err(vec![
            format!("Command '{}' not found.", input),
            "Is this a typo or just wishful thinking?".to_string(),
        ]),
    }
}