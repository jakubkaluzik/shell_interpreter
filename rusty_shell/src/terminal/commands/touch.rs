use crate::terminal::commands::common::*;

pub fn execute_touch(display: &mut Display, command: Command) -> Result<(), CommandError> {
    if let Command::Touch {verbose, targets } = command {
        for tgrt in targets {
            let mut new_trgt = PathBuf::from(&display.curr_dir);
            new_trgt.push(&tgrt);

            match File::create(&new_trgt) {
                Ok(_) => {
                    if verbose { 
                        display.output.push(format!("Created file: {:?}", new_trgt)) 
                    }
                },
                Err(e) => {
                    match e.kind() {
                        io::ErrorKind::NotFound => {
                            return Err(CommandError::ParentDirectoryDoesNotExist { command: "touch", path: new_trgt });
                        }
                        io::ErrorKind::PermissionDenied => {
                            return Err(CommandError::PermissionDenied { command: "touch", path: new_trgt });
                        }
                        _ => {
                            return Err(CommandError::FailedToCreateFile { command: "touch", path: new_trgt });
                        }
                    }
                }
            }
        }
    }
    Ok(())
}