use crate::terminal::cmd_defs::{Command, CommandError};

pub fn parse(dir: &String, input: &String) -> Command {
    let mut parts = input.split_whitespace();
    match parts.next() {
        Some("cd") => {
            if let Some(target) = parts.next() {
                if parts.next().is_none() {
                    Command::Cd(target.to_string())
                } else {
                    Command::Err(CommandError::TooManyArguments { command: "cd" })
                }
            } else {
                Command::Err(CommandError::NoTargetDirectory { command: "cd" })
            }
        }
        Some("ls") => Command::Ls,
        Some("clear") => Command::Clear,
        _ => Command::Err(CommandError::CommandNotFound { command: "err", input: input.clone() }),
    }
}