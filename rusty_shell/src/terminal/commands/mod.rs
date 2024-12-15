mod common;

mod ls;
mod clear;
mod cd;
mod mkdir;

use crate::terminal::commands::common::*;

pub fn execute_command(app_state: &mut AppState, result: Result<Command, CommandError>) -> Result<(), CommandError> {
    match result {
        Ok(command) => match command {
            Command::Cd { path } => cd::execute_cd(app_state, path),
            Command::Mkdir{..} => mkdir::execute_mkdir(app_state, command),
            Command::Ls => ls::execute_ls(app_state),
            Command::Clear => clear::execute_clear(app_state),
            //Command::Mkdir { path } => mkdir::execute_mkdir(app_state, path),
        },
        Err(err) => {
          Err(err)
        }
    }
    
}