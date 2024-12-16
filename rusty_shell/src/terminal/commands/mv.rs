use crate::terminal::commands::common::*;

pub fn execute_mv(display: &mut Display, command: Command) -> Result<(), CommandError> {
    if let Command::Mv {force, verbose, sources, target } = command {
        let mut errors: Vec<CommandError> = Vec::new();
        let mut new_target = PathBuf::from(&display.curr_dir);
        new_target.push(&target);
        if !new_target.exists() {
            errors.push(CommandError::FailedToResolvePath{ command: "mv", path: new_target });
            return Err(CommandError::ManyErrors(errors));
        }
        for src in sources {
            let mut new_src = PathBuf::from(&display.curr_dir);
            new_src.push(&src);
            let mut final_target = new_target.clone();
            final_target.push(new_src.file_name().unwrap());
            if new_src.exists() {
                if new_src.is_dir() {
                    let result = if force {
                        fs::rename(&new_src, &final_target)
                    } 
                    else {
                        fs::rename(&new_src, &final_target)
                    };
                    match result {
                        Ok(_) => {
                            if verbose {
                                display.output.push(format!("[ACTION]&mv: Directory '{}' moved to '{}'.", src, target));
                                display.output.push(format!("==> source: '{}'", new_src.display()));
                                display.output.push(format!("==> target: '{}'", final_target.display()));
                            }
                        }
                        Err(e) => {
                            match e.kind() {
                                io::ErrorKind::NotFound => {
                                    errors.push(CommandError::DirectoryDoesNotExist { command: "mv", path: new_src });
                                }
                                io::ErrorKind::PermissionDenied => {
                                    errors.push(CommandError::PermissionDenied { command: "mv", path: new_src });
                                }
                                _ => {
                                    errors.push(CommandError::FailedToMoveDirectory { command: "mv", path: new_src });
                                }
                            }
                        }
                    }
                } else {
                    let result = if force {
                        fs::rename(&new_src, &final_target)
                    } 
                    else {
                        fs::rename(&new_src, &final_target)
                    };
                    match result {
                        Ok(_) => {
                            if verbose {
                                display.output.push(format!("[ACTION]&mv: File '{}' moved to '{}'.", src, target));
                                display.output.push(format!("==> source: '{}'", new_src.display()));
                                display.output.push(format!("==> target: '{}'", final_target.display()));
                            }
                        }
                        Err(e) => {
                            match e.kind() {
                                io::ErrorKind::NotFound => {
                                    errors.push(CommandError::FileDoesNotExist { command: "mv", path: new_src });
                                }
                                io::ErrorKind::PermissionDenied => {
                                    errors.push(CommandError::PermissionDenied { command: "mv", path: new_src });
                                }
                                _ => {
                                    errors.push(CommandError::FailedToMoveFile { command: "mv", path: new_src });
                                }
                            }
                        }
                    }
                }
            } 
            else {
                errors.push(CommandError::FailedToResolvePath{ command: "mv", path: new_src });
            }
        }
        if !errors.is_empty() {
            return Err(CommandError::ManyErrors(errors));
        }
    }
    Ok(())
}