use crate::terminal::commands::prelude::*;

pub fn execute_clear(app_state: &mut AppState) -> Command {
    app_state.scroll = 0;
    app_state.curr_count_lines = 0;
    app_state.output.clear();
    Command::Ok
}