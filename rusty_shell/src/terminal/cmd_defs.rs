use std::path::PathBuf;
use clap::{Parser, Subcommand};
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
        /// Print information about the files created
        verbose: bool,
        #[arg(required = true)]
        /// The file to create
        targets: Vec<String>,
    },
    Mv {
        #[arg(short, long)]
        /// Overwrite existing files
        force: bool,
        #[arg(short, long)]
        /// Print information about the directories/files moved
        verbose: bool,
        #[arg(required = true)]
        /// The source directories/files
        sources: Vec<String>,
        #[arg(required = true)]
        /// The destination directory
        target: String,
    },
    Cp {
        #[arg(short, long)]
        /// Copy directories recursively
        recursive: bool,
        #[arg(short, long)]
        /// Overwrite existing files
        force: bool,
        #[arg(short, long)]
        /// Print information about the directories/files copied
        verbose: bool,
        #[arg(required = true)]
        /// The source directories/files
        sources: Vec<String>,
        #[arg(required = true)]
        /// The destination directory
        target: String,
    },
    Rm {
        #[arg(short, long)]
        /// Remove directories recursively
        recursive: bool,
        #[arg(short, long)]
        /// Force ignores nonexistent files and missing operands without error
        force: bool,
        #[arg(short, long)]
        /// Print information about the directories/files removal
        verbose: bool,
        #[arg(required = true)]
        /// The directories/files to remove
        targets: Vec<String>,
    },
    Echo {
        // Allows newline character
        //#[arg(short, long)]
        //escape: bool,
        #[arg(required = true)]
        /// The string to display
        text: String,
    },
    Cat {
        #[arg(required = true, last = true)]
        /// The file to display
        path: String,
    },
    Ls,
    Clear,
}
impl Command {
    fn regex_check(re: Regex, cmd: &'static str, input: &String) -> Result<(), CommandError> {
        if !re.is_match(input) {
            return Err(CommandError::IncorrectArgumentOrder { command: cmd, input: input.clone() });
        }
        Ok(())
    }
    pub fn validate_order(&self, input: String) -> Result<(), CommandError> {
        match self {
            Command::Cd { .. } => Ok(()),
            Command::Mkdir { .. } => Command::regex_check(Regex::new(r"^mkdir\s+((?:-\s+\S+|--\s+\S+)\s+)?.*$").unwrap(),"mkdir", &input),
            Command::Touch { .. } => Command::regex_check(Regex::new(r"^touch\s+((?:-\s+\S+|--\s+\S+)\s+)?.*$").unwrap(),"touch", &input),
            Command::Cp { .. } => Command::regex_check(Regex::new(r"^cp\s+((?:-\s+\S+|--\s+\S+)\s+)?.*$").unwrap(),"cp", &input),
            Command::Mv { .. } => Command::regex_check(Regex::new(r"^mv\s+((?:-\s+\S+|--\s+\S+)\s+)?.*$").unwrap(),"mv", &input),
            Command::Rm { .. } => Command::regex_check(Regex::new(r"^rm\s+((?:-\s+\S+|--\s+\S+)\s+)?.*$").unwrap(),"rm", &input),
            Command::Echo { .. } => Command::regex_check(Regex::new(r"^echo\s+((?:-\s+\S+|--\s+\S+)\s+)?.*$").unwrap(),"echo", &input),
            Command::Cat { .. } => Ok(()),
            Command::Ls => Ok(()),
            Command::Clear => Ok(()),
        }
    }
}
pub enum CommandError {
    //Parsing related
    CommandNotFound { command: String, input: String },
    TooManyArguments { command: String, input: String },
    MissingRequiredArgument { command: String, input: String },
    IncorrectArgumentOrder { command: &'static str, input: String },
    UnknownArgument { command: String, input: String },
    
    //Directory related
    FailedToChangeDirectory { command: &'static str, path: PathBuf },
    FailedToCreateDirectory { command: &'static str,dir: String, path: PathBuf },
    DirectoryDoesNotExist { command: &'static str, path: PathBuf },
    NotADirectory { command: &'static str, path: PathBuf },
    DirectoryAlreadyExists { command: &'static str, dir: String, path: PathBuf },
    PermissionDenied { command: &'static str, path: PathBuf },
    ParentDirectoryDoesNotExist { command: &'static str, path: PathBuf },
    FailedToRemoveDirectory { command: &'static str, path: PathBuf },
    FailedToCopyDirectory { command: &'static str, path: PathBuf },
    FailedToMoveDirectory { command: &'static str, path: PathBuf },

    //File related
    FileDoesNotExist { command: &'static str, path: PathBuf },
    FailedToRemoveFile { command: &'static str, path: PathBuf },
    FailedToCopyFile { command: &'static str, path: PathBuf },
    FailedToCreateFile { command: &'static str, path: PathBuf },
    FailedToMoveFile { command: &'static str, path: PathBuf },

    //Path related
    FailedToConvertPath { command: &'static str, path: PathBuf },
    FailedToResolvePath { command: &'static str, path: PathBuf },
    ManyErrors(Vec<CommandError>),
}

impl CommandError {
    pub fn to_vector(self) -> Vec<String> {
        let mut err = Vec::new();
        err.push(format!("\n"));
        match self {
            //Parsing related
            CommandError::CommandNotFound { command, input } => {
                err.push(format!("[ERROR]&{}: Command not found.", command));
                err.push(format!("==> input: {}", input));
                err.push(format!("==> Is this a typo or just wishful thinking?"));
            }
            CommandError::TooManyArguments { command , input} => {
                err.push(format!("[ERROR]&{}: Too many arguments.", command));
                err.push(format!("==> input: {}", input));
            }
            CommandError::MissingRequiredArgument { command, input } => {
                err.push(format!("[ERROR]&{}: Missing required argument.", command));
                err.push(format!("==> input: {}", input));
            }
            CommandError::IncorrectArgumentOrder { command, input } => {
                err.push(format!("[ERROR]&{}: Incorrect argument order.", command));
                err.push(format!("==> input: {}", input));
            }
            CommandError::UnknownArgument { command, input } => {
                err.push(format!("[ERROR]&{}: Unknown argument.", command));
                err.push(format!("==> input: {}", input));
            }
            //Directory related
            CommandError::FailedToChangeDirectory { command, path } => {
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to change directory.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToCreateDirectory { command,dir,  path } => {
                err.clear();
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to create '{}' directory.", command, dir));
                err.push(format!("==> dir: '{}'", path.display()));
            }
            CommandError::DirectoryDoesNotExist { command, path } => {
                err.push(format!("[ERROR]&{}: Directory does not exist.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::NotADirectory { command, path } => {
                err.push(format!("[ERROR]&{}: Not a directory.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::DirectoryAlreadyExists { command, dir, path } => {
                err.clear();
                err.push(format!("[ERROR]&{}: Directory '{}' already exist.", command, dir));
                err.push(format!("==> target: '{}'", path.display()));
            }
            CommandError::PermissionDenied { command, path } => {
                err.clear();
                err.push(format!("[ERROR]&{}: Permission denied.", command));
                err.push(format!("==> target: '{}'", path.display()));
            }
            CommandError::ParentDirectoryDoesNotExist { command, path } => {
                err.clear();
                err.push(format!("[ERROR]&{}: Parent directory does not exist.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToRemoveDirectory { command, path } => {
                err.clear();
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to remove directory.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToCopyDirectory { command, path } => {
                err.clear();
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to copy directory.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToMoveDirectory { command, path } => {
                err.clear();
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to move directory.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            //File related
            CommandError::FileDoesNotExist { command, path } => {
                err.push(format!("[ERROR]&{}: File does not exist.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToRemoveFile { command, path } => {
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to remove file.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToCopyFile { command, path } => {
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to copy file.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToCreateFile { command, path } => {
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to create file.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToMoveFile { command, path } => {
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to move file.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            //Path related
            CommandError::FailedToConvertPath { command, path } => {
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to convert path.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::FailedToResolvePath { command, path } => {
                err.push(format!("[SYSTEM_ERROR]&{}: Failed to resolve path.", command));
                err.push(format!("==> path: '{}'", path.display()));
            }
            CommandError::ManyErrors(errors) => {
                for error in errors {
                    err.extend(error.to_vector());
                }
            }
        }
        err.push(format!("\n"));
        err
    }
}