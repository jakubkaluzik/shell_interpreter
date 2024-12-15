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
            Command::Cd { path } => cd::execute_cd(&mut app_state.curr_dir, path),
            Command::Mkdir{..} => mkdir::execute_mkdir(app_state, command),
            Command::Touch{..} => touch::execute_touch(&mut app_state.curr_dir,&mut app_state.output, command),

            Command::Mv {..} => mv::execute_mv(app_state, command),
            Command::Cp{..} => cp::execute_cp(app_state, command),
            Command::Rm {..} => rm::execute_rm(app_state, command),
            
            Command::Echo { text } => echo::execute_echo(app_state, text),
            Command::Cat { path } => cat::execute_cat(app_state, path),
            Command::Ls => ls::execute_ls(app_state),
            Command::Clear => clear::execute_clear(app_state),
        },
        Err(err) => {
          Err(err)
        }
    }
    
}