use crate::terminal::commands::common::*;

pub fn execute_cat(display: &mut Display, command: Command) -> Result<(), CommandError> {
    let mut out: Vec<String> = Vec::new();
    if let Command::Cat { number, blanknon, squeeze, files, output } = command {
        let mut all_lines: Vec<String> = Vec::new();
        for path in files {
            let path = PathBuf::from(path);
            if path.exists() {
                if path.is_dir() {
                    return Err(CommandError::IsDirectory { command: "cat", path: path });
                }
                let file = fs::read_to_string(&path).unwrap();
                let mut lines = file.lines();
                while let Some(line) = lines.next() {
                    all_lines.push(line.to_string());
                }
            } 
            else {
                return Err(CommandError::FileDoesNotExist { command: "cat", path: path });
            }
        }

        let mut line_number = 1;
        let mut prev_line = String::new();
        let mut prev_line_printed = false;
        for line in all_lines {
            if squeeze && line.is_empty() {
                continue;
            }

            if blanknon && line.is_empty() && prev_line.is_empty() {
                if !prev_line_printed {
                    out.push(prev_line.clone());
                    prev_line_printed = true;
                }
            } else {
                if number && !(blanknon && line.is_empty()) {
                    out.push(format!("{:<6} {}", line_number, line));
                    line_number += 1;
                } else {
                    out.push(line.to_string());
                }
                prev_line = line.to_string();
                prev_line_printed = false;
            }
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