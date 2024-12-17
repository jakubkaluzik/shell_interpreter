use std::time::SystemTime;
use chrono::{Local, TimeZone};

use crate::terminal::commands::common::*;

pub fn execute_ls(display: &mut Display, command: Command) -> Result<(), CommandError> {
    let mut out: Vec<String> = Vec::new();
    if let Command::Ls { all, long, dir, output } = command {
        let mut new_trgt = PathBuf::from(&display.curr_dir);
        if let Some(dir) = dir {
            new_trgt.push(dir);
        }
        if !new_trgt.exists() {
            return Err(CommandError::FailedToResolvePath{ command: "ls", path: new_trgt });
        }
        if new_trgt.is_dir() {
            let mut entries: Vec<_> = fs::read_dir(new_trgt).unwrap().map(|res| res.unwrap()).collect();
            entries.sort_by_key(|dir| dir.path());
            for entry in entries {
                let path = entry.path();
                if all || path.file_name().unwrap().to_str().unwrap().chars().next().unwrap() != '.' {
                    if long {
                        let metadata = fs::metadata(&path).unwrap();
                        let file_type = metadata.file_type();
                        let file_name = path.file_name().unwrap().to_str().unwrap();
                        let file_size = metadata.len();
                        let file_size = if file_type.is_dir() {
                            "dir".to_string()
                        } else {
                            file_size.to_string()
                        };
                        let file_date = metadata.modified().unwrap();
                        let file_date = file_date.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                        let file_date = Local.timestamp_opt(file_date as i64, 0).single().unwrap().format("%H:%M:%S %d-%m-%Y").to_string();
                        out.push(format!("{:<10} {:<10} {:<15}", file_size, file_date, file_name));
                    } else {
                        out.push(path.file_name().unwrap().to_str().unwrap().to_string());
                    }
                }
            }
        } else {
            return Err(CommandError::NotADirectory { command: "ls", path: new_trgt });
        }
        if let Some(output) = output {
            if let Err(_) = output_redirect(out, output.clone()) {
                return Err(CommandError::OutputRedirectFailed { command: "ls", path: output});
            }
        }
        else {
            display.output.extend(out);
        }
    }
    Ok(())
}