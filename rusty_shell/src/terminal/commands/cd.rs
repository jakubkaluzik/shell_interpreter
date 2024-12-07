use crate::terminal::AppState;
use crate::terminal::commands::Command;

use std::env;
use std::path::PathBuf;


pub fn execute_cd(app_state: &mut AppState, argument: String) -> Command {
    let mut new_dir = PathBuf::from(&app_state.curr_dir);
    new_dir.push(&argument);

    if new_dir.exists() {
        if new_dir.is_dir() {
            match new_dir.canonicalize() {
                Ok(canonical_path) => {
                    match canonical_path.into_os_string().into_string() {
                        Ok(new_dir_str) => {
                            let new_dir_str = if new_dir_str.starts_with(r"\\?\") {
                                new_dir_str[4..].to_string()
                            } else {
                                new_dir_str
                            };
    
                            app_state.curr_dir = new_dir_str;
                            if env::set_current_dir(&new_dir).is_ok() {
                                Command::Ok
                            } else {
                                Command::Err(vec![format!("cd: failed to change directory to '{}'", new_dir.display())])
                            }
                        }
                        Err(_) => Command::Err(vec![format!("cd: failed to convert path '{}' to string", new_dir.display())]),
                    }
                }
                Err(_) => Command::Err(vec![format!("cd: failed to resolve directory path '{}'", new_dir.display())]),
            }
        }
        else {
            Command::Err(vec![format!("cd: '{}' is not a directory", new_dir.display())])
        }
    } else {
        Command::Err(vec![format!("cd: directory '{}' does not exist", new_dir.display())])
    }
}