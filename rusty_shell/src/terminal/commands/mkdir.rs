use crate::terminal::commands::common::*;

pub fn execute_mkdir(app_state: &mut AppState, command: Command) -> Result<(), CommandError> {

   if let Command::Mkdir { parents, verbose, dirs } = command {
      for dir in dirs {
         let mut new_dir = PathBuf::from(&app_state.curr_dir);
         new_dir.push(&dir);

         if new_dir.exists() {
            return Err(CommandError::DirectoryAlreadyExists {command: "mkdir", dir, path: new_dir.clone()});
         } 
         else {
            let result = if parents {
               fs::create_dir_all(&new_dir)
            } else {
               fs::create_dir(&new_dir)
            };

            match result {
               Ok(_) => {
                  if verbose {
                        app_state.output.push(format!("Directory '{}' created.", dir));
                        app_state.output.push(format!("==> path: '{}'", new_dir.display()));
                  }
               }
               Err(e) => {
                  match e.kind() {
                     io::ErrorKind::PermissionDenied => {
                        return Err(CommandError::PermissionDenied {command: "mkdir", path: new_dir});
                     }
                     io::ErrorKind::AlreadyExists => {
                        return Err(CommandError::DirectoryAlreadyExists {command: "mkdir", dir, path: new_dir});
                     }
                     _ => {
                        return Err(CommandError::FailedToCreateDirectory {command: "mkdir",path: new_dir, error: e.to_string()});
                     }
                  }
               }
            }
         }
      }
   }
   Ok(())
}