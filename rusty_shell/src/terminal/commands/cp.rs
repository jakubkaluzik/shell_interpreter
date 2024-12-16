use crate::terminal::commands::common::*;

fn copy_dir(src: &Path, dst: &Path, force: &bool) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path, force)?;
        } else {
            if *force || !dst_path.exists() {
                fs::copy(&src_path, &dst_path)?;
            }
        }
    }
    Ok(())
}

pub fn execute_cp(display: &mut Display, command: Command) -> Result<(), CommandError> {
    if let Command::Cp { recursive, force, verbose, sources, target } = command {
        let mut errors: Vec<CommandError> = Vec::new();
        let mut new_target = PathBuf::from(&display.curr_dir);
        new_target.push(&target);
        if !new_target.exists() {
            errors.push(CommandError::FailedToResolvePath{ command: "cp", path: new_target });
            return Err(CommandError::ManyErrors(errors));
        }
        for src in sources {
            let mut new_src = PathBuf::from(&display.curr_dir);
            new_src.push(&src);
            let mut final_target = new_target.clone();
            final_target.push(new_src.file_name().unwrap());
            if new_src.exists() {
                if new_src.is_dir() {
                    let result = if recursive {
                        copy_dir(&new_src, &final_target, &force)
                    } else {
                        copy_dir(&new_src, &final_target, &force)
                    };
                    match result {
                        Ok(_) => {
                            if verbose {
                                display.output.push(format!("[ACTION]&cp: Directory '{}' copied to '{}'.", src, target));
                                display.output.push(format!("==> source: '{}'", new_src.display()));
                                display.output.push(format!("==> target: '{}'", final_target.display()));
                            }
                        }
                        Err(e) => {
                            match e.kind() {
                                io::ErrorKind::NotFound => {
                                    errors.push(CommandError::DirectoryDoesNotExist { command: "cp", path: new_src });
                                }
                                io::ErrorKind::PermissionDenied => {
                                    errors.push(CommandError::PermissionDenied { command: "cp", path: new_src });
                                }
                                _ => {
                                    errors.push(CommandError::FailedToCopyDirectory { command: "cp", path: new_src });
                                }
                            }
                        }
                    }
                } else if new_src.is_file() {
                    let result = fs::copy(&new_src, &final_target);
                    match result {
                        Ok(_) => {
                            if verbose {
                                display.output.push(format!("[ACTION]&cp: File '{}' copied to '{}'.", src, target));
                                display.output.push(format!("==> source: '{}'", new_src.display()));
                                display.output.push(format!("==> target: '{}'", final_target.display()));
                            }
                        }
                        Err(e) => {
                            match e.kind() {
                                io::ErrorKind::NotFound => {
                                    errors.push(CommandError::FileDoesNotExist { command: "cp", path: new_src });
                                }
                                io::ErrorKind::PermissionDenied => {
                                    errors.push(CommandError::PermissionDenied { command: "cp", path: new_src });
                                }
                                _ => {
                                    errors.push(CommandError::FailedToCopyFile { command: "cp", path: new_src });
                                }
                            }
                        }
                    }
                }
            } else {
                errors.push(CommandError::FailedToResolvePath { command: "cp", path: new_src });
            }
        }
        if !errors.is_empty() {
            return Err(CommandError::ManyErrors(errors));
        }
    }
    Ok(())
}