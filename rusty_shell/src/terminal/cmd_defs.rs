use std::{collections::VecDeque, path::{Path, PathBuf}};
use clap::{ArgGroup,Parser, Subcommand};
use regex::Regex;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    Cd {
        /// The path to change to
        path: String,
    },
    Mkdir {
        #[arg(short, long)]
        /// Create parent directories if they do not exist
        parents: bool,
        #[arg(short, long)]
        /// Print information about the directories created
        verbose: bool,
        #[arg(required = true)]
        /// The directory/ies to create
        dirs: Vec<String>,
    },
    Touch {
        #[arg(short, long)]
        timestamp: Option<String>,
        #[arg(required = true)]
        /// The file to create
        targets: Vec<String>,
    },
    Mv {
        #[arg(short, long)]
        /// Move directories recursively
        recursive: bool,
        #[arg(short, long)]
        /// Overwrite existing files
        force: bool,
        #[arg(required = true)]
        /// The source directory
        source: Vec<String>,
        #[arg(required = true)]
        /// The destination directory
        destination: String,
    },
    Cp {
        #[arg(short, long)]
        /// Copy directories recursively
        recursive: bool,
        #[arg(short, long)]
        /// Overwrite existing files
        force: bool,
        #[arg(required = true)]
        /// The source directory
        source: Vec<String>,
        #[arg(required = true)]
        /// The destination directory
        destination: String,
    },
    Rm {
        #[arg(short, long)]
        /// Remove directories recursively
        recursive: bool,
        #[arg(short, long)]
        /// Force removal without confirmation
        force: bool,
        #[arg(required = true)]
        /// The directory/ies to remove
        dirs: Vec<String>,
    },
    Echo {
        #[arg(required = true)]
        /// The string to display
        text: String,
    },
    Cat {
        #[arg(required = true)]
        /// The file to display
        path: String,
    },
    Ls,
    Clear,
}
impl Command {
    pub fn validate_order(&self, input: String) -> Result<(), CommandError> {
        match self {
            Command::Cd { .. } => Ok(()),
            Command::Mkdir { .. } => Ok(()),
            Command::Touch { .. } => {
                let re = Regex::new(r"^touch\s+((?:-t\s+\S+|--timestamp\s+\S+)\s+)?.*$").unwrap();
                if !re.is_match(&input) {
                    return Err(CommandError::IncorrectArgumentOrder { command: "touch", input: input.clone() });
                }
                Ok(())
            },
            Command::Mv { .. } => Ok(()),
            Command::Cp { .. } => Ok(()),
            Command::Rm { .. } => Ok(()),
            Command::Echo { .. } => Ok(()),
            Command::Cat { .. } => Ok(()),
            Command::Ls => Ok(()),
            Command::Clear => Ok(()),
        }
    }
}
pub enum CommandError {
    //Parsing related
    CommandNotFound { command: String, input: String },
    NoTargetDirectory { command: String },
    TooManyArguments { command: String, input: String },
    MissingRequiredArgument { command: String },
    IncorrectArgumentOrder { command: &'static str, input: String },
    //Directory related
    FailedToChangeDirectory { command: &'static str, path: PathBuf },
    FailedToCreateDirectory { command: &'static str, path: PathBuf, error: String },
    DirectoryDoesNotExist { command: &'static str, path: PathBuf },
    NotADirectory { command: &'static str, path: PathBuf },
    DirectoryAlreadyExists { command: &'static str, dir: String, path: PathBuf },
    PermissionDenied { command: &'static str, path: PathBuf },
    //Path related
    FailedToConvertPath { command: &'static str, path: PathBuf },
    FailedToResolvePath { command: &'static str, path: PathBuf },
}

impl CommandError {
    pub fn to_vector(self) -> VecDeque<String> {
        let mut err = VecDeque::new();
        match self {
            //Parsing related
            CommandError::CommandNotFound { command, input } => {
                err.push_back(format!("[ERROR]&{}: Command not found.", command));
                err.push_back(format!("==> input: {}", input));
                err.push_back(format!("==> Is this a typo or just wishful thinking?"));
            }
            CommandError::NoTargetDirectory { command } => {
                err.push_back(format!("[ERROR]&{}: No target directory specified.", command));
            }
            CommandError::TooManyArguments { command , input} => {
                err.push_back(format!("[ERROR]&{}: Too many arguments.", command));
                err.push_back(format!("==> input: {}", input));
            }
            CommandError::MissingRequiredArgument { command } => {
                err.push_back(format!("[ERROR]&{}: Missing required argument.", command));
            }
            CommandError::IncorrectArgumentOrder { command, input } => {
                err.push_back(format!("[ERROR]&{}: Incorrect argument order.", command));
                err.push_back(format!("==> input: {}", input));
            }
            //Directory related
            CommandError::FailedToChangeDirectory { command, path } => {
                err.push_back(format!("[SYSTEM_ERROR]&{}: Failed to change directory.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToCreateDirectory { command, path, error } => {
                err.push_back(format!("[SYSTEM_ERROR]&{}: Failed to create directory.", command));
                err.push_back(format!("==> dir: '{}'", path.display()));
                err.push_back(format!("==> err: '{}'", error));
            }
            CommandError::DirectoryDoesNotExist { command, path } => {
                err.push_back(format!("[ERROR]&{}: Directory does not exist.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
            CommandError::NotADirectory { command, path } => {
                err.push_back(format!("[ERROR]&{}: Not a directory.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
            CommandError::DirectoryAlreadyExists { command, dir, path } => {
                err.push_back(format!("[ERROR]&{}: Directory/ies '{}' already exist.", command, dir));
                err.push_back(format!("==> dir: '{}'", dir));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
            CommandError::PermissionDenied { command, path } => {
                err.push_back(format!("[ERROR]&{}: Permission denied.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
            //Path related
            CommandError::FailedToConvertPath { command, path } => {
                err.push_back(format!("[SYSTEM_ERROR]&{}: Failed to convert path.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToResolvePath { command, path } => {
                err.push_back(format!("[SYSTEM_ERROR]&{}: Failed to resolve path.", command));
                err.push_back(format!("==> path: '{}'", path.display()));
            }
        }
        err.push_front(format!("\n"));
        err.push_back(format!("\n"));
        err
    }
}