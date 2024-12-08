use std::path::PathBuf;

pub enum Command {
    Cd(String),
    Ls,
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
    pub fn to_vector(self) -> Vec<String> {
        match self {
            CommandError::CommandNotFound { command, input } => {
                vec![
                    format!("[ERROR]&{}: Command not found.", command),
                    format!("==> input: {}", input),
                    format!("==> Is this a typo or just wishful thinking?"),
                ]
            }
            CommandError::TooManyArguments { command } => {
                vec![
                    format!("[ERROR]&{}: Too many arguments.", command),
                ]
            }
            CommandError::NoTargetDirectory { command } => {
                vec![
                    format!("[ERROR]&{}: No target directory specified.", command),
                ]
            }
            CommandError::FailedToChangeDirectory { command, path } => {
                vec![
                    format!("[SYSTEM_ERROR]&{}: Failed to change directory.", command),
                    format!("==> path: '{}'", path.display()),
                ]
            }
            CommandError::FailedToConvertPath { command, path } => {
                vec![
                    format!("[SYSTEM_ERROR]&{}: Failed to convert path to string.", command),
                    format!("==> path: {}", path.display()),
                ]
            }
            CommandError::FailedToResolvePath { command, path } => {
                vec![
                    format!("[SYSTEM_ERROR]&{}: Failed to resolve directory path.", command),
                    format!("==> path: {}", path.display()),
                ]
            }
            CommandError::NotADirectory { command, path } => {
                vec![
                    format!("[ERROR]&{}: Not a directory.", command),
                    format!("==> path: {}", path.display()),
                ]
            }
            CommandError::DirectoryDoesNotExist { command, path } => {
                vec![
                    format!("[ERROR]&{}: Directory does not exist.", command),
                    format!("==> path: {}", path.display()),
                    format!("\n")
                ]
            }
        }
    }
}