mod common;

mod ls;
mod clear;
mod cd;
mod mkdir;

use crate::terminal::commands::common::*;

pub fn execute_command(app_state: &mut AppState, command: Command) -> Command {
    match command {
        Command::Cd(argument) => cd::execute_cd(app_state, argument),
        Command::Ls => ls::execute_ls(app_state),
        //Command::Mkdir{_} => Command::Ok,
        Command::Clear => clear::execute_clear(app_state),
        Command::Err(_) => command,
        Command::Ok => command,
    }
    
}