use crate::terminal::commands::common::*;

pub fn execute_clear(app_state: &mut AppState) -> Result<(), CommandError> {
    app_state.scroll.curr_scroll = 0;
    app_state.scroll.curr_count_lines = 0;
    app_state.past.is_displayed = false;
    app_state.past.curr_prev_input = app_state.past.prev_inputs.len();
    app_state.display.output.clear();
    Ok(())
}