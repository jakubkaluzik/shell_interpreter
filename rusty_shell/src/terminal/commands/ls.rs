use crate::terminal::commands::common::*;

pub fn execute_ls(display: &mut Display) -> Result<(), CommandError> {
    let mut output = Vec::new();
    let dir = std::fs::read_dir(&display.curr_dir).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        output.push(file_name.to_string());
    }
    display.output.extend(output);
    Ok(())
}