use crate::terminal::commands::Command;
use crate::terminal::AppState;

pub fn execute_ls(app_state: &mut AppState) -> Command {
    let mut output = Vec::new();
    let dir = std::fs::read_dir(&app_state.curr_dir).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        output.push(file_name.to_string());
    }
    app_state.output.extend(output);
    Command::Ok
}