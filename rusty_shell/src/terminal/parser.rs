use crate::terminal::cmd_defs::{Cli,Command, CommandError};
use clap::Parser;
use clap::error::ErrorKind;

pub fn parse(input: String) -> Result<Command, CommandError> {
    let mut args: Vec<&str> = vec!["rusty_shell"];
    args.extend(input.split_whitespace());
    let err_cmd = args.get(1).unwrap_or(&"err").to_string();
    match Cli::try_parse_from(args) {
        Ok(cli) => {
            match cli.command.validate_order(input.clone()) {
                Ok(_) => Ok(cli.command),
                Err(err) => Err(err),
            }
        }
        Err(err) => {
            match err.kind() {
                ErrorKind::TooManyValues => Err(CommandError::TooManyArguments { command: err_cmd, input: input.clone() }),
                ErrorKind::MissingRequiredArgument => Err(CommandError::MissingRequiredArgument { command: err_cmd, input: input.clone() }),
                ErrorKind::UnknownArgument => Err(CommandError::UnknownArgument { command: err_cmd, input: input.clone() }),
                _ => Err(CommandError::CommandNotFound { command: err_cmd, input: input.clone() })
            }
        }
    }
}