use crate::terminal::commands::common::*;
use std::fs;

pub fn execute_mkdir(app_state: &mut AppState,command: Command) -> Result<(), CommandError> {
   if let Command::Mkdir { parents, verbose, dirs } = command { 
      for dir in dirs {
         let mut new_dir = PathBuf::from(&app_state.curr_dir);
         new_dir.push(&dir);
         if new_dir.exists() {
            if verbose {
               println!("Directory '{}' already exists.", new_dir.display());
            }
         } else {
            if parents {
               if let Err(_) = fs::create_dir_all(&new_dir) {
                  return Err(CommandError::FailedToCreateDirectory { command: "mkdir", path: new_dir});
               }
            } else {
               if let Err(_) = fs::create_dir(&new_dir) {
                  return Err(CommandError::FailedToCreateDirectory { command: "mkdir", path: new_dir});
               }
            }
            if verbose {
               println!("Created directory '{}'.", new_dir.display());
            }
         }
      }
   }
   Ok(())
}