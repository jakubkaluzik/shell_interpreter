use crate::terminal::commands::common::*;

pub fn execute_mkdir(display: &mut Display, command: Command) -> Result<(), CommandError> {
   let mut errors = Vec::new();
   if let Command::Mkdir { parents, verbose, dirs } = command {
      for dir in dirs {
         let mut new_dir = PathBuf::from(&display.curr_dir);
         new_dir.push(&dir);

         /*if new_dir.exists() {
            return Err(CommandError::DirectoryAlreadyExists {command: "mkdir", dir, path: new_dir.clone()});
         }*/
         let result = if parents {
            fs::create_dir_all(&new_dir)
         } else {
            fs::create_dir(&new_dir)
         };

         match result {
            Ok(_) => {
               if verbose {
                  display.output.push(format!("Directory '{}' created.", dir));
                  display.output.push(format!("==> path: '{}'", new_dir.display()));
               }
            }
            Err(e) => {
               match e.kind() {
                  io::ErrorKind::PermissionDenied => {
                     errors.push(CommandError::PermissionDenied {command: "mkdir", path: new_dir});
                  }
                  io::ErrorKind::AlreadyExists => {
                     errors.push(CommandError::DirectoryAlreadyExists {command: "mkdir", dir, path: new_dir});
                  }
                  _ => {
                     errors.push(CommandError::FailedToCreateDirectory {command: "mkdir", dir,path: new_dir});
                  }
               }
            }
         }
      }
      if !errors.is_empty(){
         return Err(CommandError::ManyErrors(errors));
      }
   }
   Ok(())
}