use std::{collections::VecDeque, path::PathBuf};

pub enum Command {
    Cd(String),
    Ls,
    //Mkdir{parent: bool, print: bool, mode: String, dirs: Vec<String>},
    Clear,
    Ok,
    Err(CommandError),
}
pub enum CommandError {
    CommandNotFound { command: &'static str, input: String },
    TooManyArguments { command: &'static str },
    NoTargetDirectory { command: &'static str },
    FailedToChangeDirectory { command: &'static str, path: PathBuf },
    FailedToConvertPath { command: &'static str, path: PathBuf },
    FailedToResolvePath { command: &'static str, path: PathBuf },
    NotADirectory { command: &'static str, path: PathBuf },
    DirectoryDoesNotExist { command: &'static str, path: PathBuf },
}

impl CommandError {
    pub fn to_vector(self) -> VecDeque<String> {
        let mut err = VecDeque::new();
        match self {
            CommandError::CommandNotFound { command, input } => {
                err.push_back(format!("[ERROR]&{}: Command not found.", command));
                err.push_back(format!("==> input: {}", input));
                err.push_back(format!("==> Is this a typo or just wishful thinking?"));
            }
            CommandError::TooManyArguments { command } => {
                err.push_back(format!("[ERROR]&{}: Too many arguments.", command));
            }
            CommandError::NoTargetDirectory { command } => {
                err.push_back(format!("[ERROR]&{}: No target directory specified.", command));
            }
            CommandError::FailedToChangeDirectory { command, path } => {
                err.push_back(format!("[SYSTEM_ERROR]&{}: Failed to change directory.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToConvertPath { command, path } => {
                err.push_back(format!("[SYSTEM_ERROR]&{}: Failed to convert path.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToResolvePath { command, path } => {
                err.push_back(format!("[SYSTEM_ERROR]&{}: Failed to resolve path.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
            CommandError::NotADirectory { command, path } => {
                err.push_back(format!("[ERROR]&{}: Not a directory.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
            CommandError::DirectoryDoesNotExist { command, path } => {
                err.push_back(format!("[ERROR]&{}: Directory does not exist.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
        }
        err.push_front(format!("\n"));
        err.push_back(format!("\n"));
        err
    }
}