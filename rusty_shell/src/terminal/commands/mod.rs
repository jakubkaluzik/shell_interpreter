mod common;

mod cd;
mod mkdir;
mod touch;

mod mv;
mod cp;
mod rm;

mod echo;
mod cat;
mod ls;

mod clear;


use crate::terminal::commands::common::*;

pub fn execute_command(app_state: &mut AppState, result: Result<Command, CommandError>) -> Result<(), CommandError> {
    match result {
        Ok(command) => match command {
            Command::Cd { path } => cd::execute_cd(&mut app_state.display.curr_dir, path),
            Command::Mkdir{..} => mkdir::execute_mkdir(&mut app_state.display, command),
            Command::Touch{..} => touch::execute_touch(&mut app_state.display, command),
            Command::Cp{..} => cp::execute_cp(&mut app_state.display, command),
            Command::Mv{..} => mv::execute_mv(&mut app_state.display, command),
            Command::Rm {..} => rm::execute_rm(&mut app_state.display, command),
            Command::Echo {..} => echo::execute_echo(&mut app_state.display, command),
            Command::Cat { path } => cat::execute_cat(&mut app_state.display, path),
            Command::Ls => ls::execute_ls(&mut app_state.display),
            Command::Clear => clear::execute_clear(app_state),
        },
        Err(err) => Err(err)
    }
    
}