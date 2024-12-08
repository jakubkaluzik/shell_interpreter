mod prelude;

mod ls;
mod clear;
mod cd;

use crate::terminal::commands::prelude::*;

pub fn execute_command(app_state: &mut AppState, command: Command) -> Command {
    match command {
        Command::Cd(argument) => cd::execute_cd(app_state, argument),
        Command::Ls => ls::execute_ls(app_state),
        Command::Clear => clear::execute_clear(app_state),
        Command::Err(_) => command,
        Command::Ok => command,
    }
    
}