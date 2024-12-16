use crate::terminal::commands::common::*;
//DONE
pub fn execute_rm(display: &mut Display, command: Command) -> Result<(), CommandError> {
    if let Command::Rm {recursive, force, verbose, targets}  = command {
        let mut errors: Vec<CommandError> = Vec::new();
        for trgt in targets {
                let mut new_path = PathBuf::from(&display.curr_dir);
                new_path.push(&trgt);
            
                if new_path.exists() {
                    if new_path.is_dir() {
                        let result = if recursive {
                            fs::remove_dir_all(&new_path)
                        } else {
                            fs::remove_dir(&new_path)
                        };
                        match result {
                            Ok(_) => {
                                if verbose {
                                    display.output.push(format!("[ACTION]&rm: Directory '{}' removed.", trgt));
                                    display.output.push(format!("==> path: '{}'", new_path.display()));
                                }
                            }
                            Err(e) => {
                                match e.kind() {
                                    io::ErrorKind::NotFound => {
                                        errors.push(CommandError::DirectoryDoesNotExist { command: "rm", path: new_path });
                                    }
                                    io::ErrorKind::PermissionDenied => {
                                        errors.push(CommandError::PermissionDenied { command: "rm", path: new_path });
                                    }
                                    _ => {
                                        errors.push(CommandError::FailedToRemoveDirectory { command: "rm", path: new_path });
                                    }
                                }
                            }
                        }
                    } 
                    else if new_path.is_file() {
                        let result = fs::remove_file(&new_path);
                        match result {
                            Ok(_) => {
                                if verbose {
                                    display.output.push(format!("[ACTION]&rm: File '{}' removed.", trgt));
                                    display.output.push(format!("==> path: '{}'", new_path.display()));
                                }
                            }
                            Err(e) => {
                                match e.kind() {
                                    io::ErrorKind::NotFound => {
                                        errors.push(CommandError::FileDoesNotExist { command: "rm", path: new_path });
                                    }
                                    io::ErrorKind::PermissionDenied => {
                                        errors.push(CommandError::PermissionDenied { command: "rm", path: new_path });
                                    }
                                    _ => {
                                        errors.push(CommandError::FailedToRemoveFile { command: "rm", path: new_path });
                                    }
                                }
                            }
                        }
                    }
                    else {
                        if force {
                            continue;
                        }
                        errors.push(CommandError::FailedToResolvePath { command: "rm", path: new_path })
                    }
                } 
                else {
                    if force {
                        continue;
                    }
                    errors.push(CommandError::FailedToResolvePath { command: "rm", path: new_path });
                }
        }
        if !errors.is_empty() {
            return Err(CommandError::ManyErrors(errors));
        }
    }
    Ok(())
}