use crossterm::event::KeyEvent;
use ratatui::crossterm::event::{KeyCode, KeyEventKind};
use crate::console::AppState;

pub enum EventResult {
    Continue,
    Exit,
}

pub fn handle_events(key: KeyEvent, app_state: &mut AppState) -> EventResult {
    if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat {
        match key.code {
            KeyCode::Char(c) => {
                app_state.curr_input.push(c);
                EventResult::Continue
            }
            KeyCode::Backspace => {
                app_state.curr_input.pop();
                EventResult::Continue
            }
            KeyCode::Esc => EventResult::Exit,
            _ => EventResult::Continue,
        }
    }
    else {
        EventResult::Continue
    }
}