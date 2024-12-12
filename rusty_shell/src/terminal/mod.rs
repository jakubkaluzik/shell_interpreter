mod events;
mod parser;
mod commands;
mod state;
mod cmd_defs;

pub use state::AppState;
use state::CursorState;

pub use std::io;
use crossterm::event::{self, EnableMouseCapture};
use ratatui::{style::Stylize, DefaultTerminal};

fn set_display(app_state: &mut AppState, cursor_state: &mut CursorState) -> String {
    app_state.curr_count_lines = 0;
    let mut display = String::new();
    let dir_input = format!("{} -> {}", app_state.curr_dir, app_state.curr_input);
    let mut start: usize = 0;
    let mut end: usize = 0;
    for out in app_state.output.iter().chain(std::iter::once(&dir_input)) {
        let mut flag = false;
        start = 0;
        end = app_state.screen_area.width as usize;
        let out_len = out.len();
        while out_len > end || flag {
            display.push_str(&out[start..end]);
            app_state.curr_count_lines += 1;
            if flag {
                break;  
            }
            display.push_str("\n");
            start += app_state.screen_area.width as usize;
            end += app_state.screen_area.width as usize;
            if end >= out_len {
                end = out_len;
                flag = true;
            }
            
        }
        if out_len <= app_state.screen_area.width as usize {
            end = out_len;
            display.push_str(&out);
            app_state.curr_count_lines += 1;
        }
        if out != "\n" {
            display.push_str("\n");
        }
    }
    cursor_state.x = (&dir_input[start..end].len() + 1) as u16;
    cursor_state.y = {
        if app_state.curr_count_lines > app_state.screen_area.height {
            app_state.screen_area.height
        } 
        else {
            app_state.curr_count_lines
        }
    };
    if cursor_state.x > app_state.screen_area.width && cursor_state.y < app_state.screen_area.height {
        cursor_state.y += 1;
        cursor_state.x = 1;
    }

    display
}
fn set_offset(app_state: &mut AppState) -> u16 {
    let offset;
    if app_state.screen_area.height < app_state.curr_count_lines {
        offset = app_state.curr_count_lines - app_state.screen_area.height;
    }
    else {
        offset = 0;
    }
    offset
}
pub fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut app_state = AppState::new();
    let mut cursor_state = state::CursorState::new();

    crossterm::execute!(std::io::stdout(), EnableMouseCapture, crossterm::cursor::Show)?;

    loop {
        terminal.draw(|frame| {
            app_state.screen_area = frame.area();
            app_state.screen_area.height = app_state.screen_area.height - 2;
            app_state.screen_area.width = app_state.screen_area.width - 2;
            let display = set_display(&mut app_state, &mut cursor_state);
            let block = ratatui::widgets::Paragraph::new(display)
                .block(ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title("Rusty shell"))
                .scroll(((set_offset(&mut app_state) - app_state.scroll), 0))
                .on_black();
            frame.render_widget(block, frame.area());
            frame.set_cursor_position((cursor_state.x, cursor_state.y));
    
        })?;

        
        if let Ok(event) = event::read() {
            let result = events::handle_event(event, &mut app_state);
            match result {
                events::EventResult::Continue => {}
                events::EventResult::Parse => handle_parse(&mut app_state),
                events::EventResult::Exit => break,
                
                
            }
        }
    }
    Ok(())
}

fn handle_parse(app_state: &mut AppState) {
    let result = commands::execute_command(app_state, parser::parse(app_state.curr_input.clone()));
    match result {
        cmd_defs::Command::Err(err) => {
            app_state.output.extend(err.to_vector());
        }
        _ => {}
    }
    app_state.curr_input.clear();
}
