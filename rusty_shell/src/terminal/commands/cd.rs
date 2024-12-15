use crate::terminal::commands::common::*;

pub fn execute_cd(curr_dir: &mut String, argument: String) -> Result<(), CommandError> {
    let mut new_dir = PathBuf::from(&curr_dir);
    new_dir.push(&argument);

    if new_dir.exists() {
        if new_dir.is_dir() {
            match new_dir.canonicalize() {
                Ok(canonical_path) => {
                    match canonical_path.into_os_string().into_string() {
                        Ok(new_dir_str) => {
                            #[cfg(windows)]
                            let new_dir_str = if new_dir_str.starts_with(r"\\?\") {
                                new_dir_str[4..].to_string()
                            } else {
                                new_dir_str
                            };
    
                            *curr_dir = new_dir_str;
                            if env::set_current_dir(&new_dir).is_ok() {
                                Ok(())
                            }
                            else {
                                Err(CommandError::FailedToChangeDirectory { command: "cd", path: new_dir })
                            }
                        }
                        Err(_) => Err(CommandError::FailedToConvertPath { command: "cd", path: new_dir }),
                    }
                }
                Err(_) => Err(CommandError::FailedToResolvePath { command: "cd", path: new_dir }),
            }
        }
        else {
            Err(CommandError::NotADirectory { command: "cd", path: new_dir })
        }
    } else {
        Err(CommandError::DirectoryDoesNotExist { command: "cd", path: new_dir })
    }
}