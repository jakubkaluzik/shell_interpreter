use crate::terminal::AppState;

use crossterm::event::Event;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::crossterm::event::{KeyCode, KeyEventKind};
use crossterm::event::MouseEventKind;

pub enum EventResult {
    Continue,
    Parse,
    Exit,
}

pub fn handle_event(event: Event, app_state: &mut AppState) -> EventResult {
    match event {
        Event::Mouse(mouse_event) => handle_mouse_event(mouse_event, app_state),
        Event::Key(key_event) => handle_key_event(key_event, app_state),
        Event::Resize(_, _) => EventResult::Continue,
        _ => EventResult::Continue,
    }
}

fn handle_key_event(key: KeyEvent, app_state: &mut AppState) -> EventResult {
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
            //TODO: Fix duplicates prev inputs
            // !app_state.prev_inputs.contains(&app_state.curr_input) 
            KeyCode::Enter => {
                app_state.output.push(app_state.curr_dir.clone() + " -> " + &app_state.curr_input.clone());
                if app_state.curr_input != "" {
                    if app_state.prev_inputs.len() >= app_state.max_prev_inputs {
                        app_state.prev_inputs.pop_front();
                        app_state.prev_inputs.push_back(app_state.curr_input.clone());
                    }
                    else {
                        app_state.prev_inputs.push_back(app_state.curr_input.clone());
                        app_state.curr_prev_input += 1;
                    }
                }
                app_state.is_displayed = false;
                app_state.curr_prev_input = app_state.prev_inputs.len();
                EventResult::Parse
            }
            KeyCode::Up => {
                if !app_state.prev_inputs.is_empty() {
                    if app_state.curr_prev_input > 0 {
                        if app_state.is_displayed && app_state.curr_prev_input == app_state.prev_inputs.len() && app_state.prev_inputs.len() > 1 {
                            app_state.curr_prev_input -= 2;
                        }
                        else {
                            app_state.curr_prev_input -= 1;
                        }
                    }
                    app_state.is_displayed = true;
                    app_state.curr_input = app_state.prev_inputs[app_state.curr_prev_input].clone();
                    
                }
                EventResult::Continue
            }
            KeyCode::Down => {
                if !app_state.prev_inputs.is_empty() {
                    if app_state.curr_prev_input < app_state.prev_inputs.len(){
                        if app_state.is_displayed && app_state.curr_prev_input == 0 && app_state.prev_inputs.len() > 1 {
                            app_state.curr_prev_input += 2;
                        }
                        else {
                            app_state.curr_prev_input += 1;
                        }
                    }
                    app_state.is_displayed = true;
                    app_state.curr_input = app_state.prev_inputs[app_state.curr_prev_input - 1].clone();
                }
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
fn handle_mouse_event(mouse_event: MouseEvent, app_state: &mut AppState) -> EventResult {
    match mouse_event.kind {
        MouseEventKind::ScrollUp => {
            if app_state.curr_count_lines > app_state.screen_area.height && (app_state.curr_count_lines - app_state.screen_area.height) > 0 {
                if app_state.scroll < app_state.curr_count_lines - app_state.screen_area.height {
                    app_state.scroll += 1;
                }
            }
        }
        MouseEventKind::ScrollDown => {
            if app_state.scroll > 0 {
                app_state.scroll -= 1;
            }
        }
        _ => {}
    }
    EventResult::Continue
}