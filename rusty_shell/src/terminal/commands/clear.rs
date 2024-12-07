use crate::terminal::AppState;
use crate::terminal::commands::Command;

pub fn execute_clear(app_state: &mut AppState) -> Command {
    app_state.scroll = 0;
    app_state.curr_count_lines = 0;
    app_state.output.clear();
    Command::Ok
}