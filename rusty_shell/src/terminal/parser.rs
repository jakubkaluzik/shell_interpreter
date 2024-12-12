use crate::terminal::cmd_defs::{Command, CommandError};

pub fn parse(input: String) -> Command {
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
        /*Some("mkdir") => {
            let mut parent = false;
            let mut print = false;
            let mut mode = String::new();
            let mut dirs = Vec::new();
            for part in parts {
                match part {
                    "-p" => parent = true,
                    "-v" => print = true,
                    "-m" => {
                        if let Some(mode_str) = parts.next() {
                            mode = mode_str.to_string();
                        } else {
                            return Command::Err(CommandError::TooManyArguments { command: "mkdir" });
                        }
                    }
                    _ => dirs.push(part.to_string()),
                }
            }
            Command::Mkdir { parent, print, mode, dirs }
        }
        */
        Some("clear") => Command::Clear,
        _ => Command::Err(CommandError::CommandNotFound { command: "err", input: input.clone() }),
    }
}