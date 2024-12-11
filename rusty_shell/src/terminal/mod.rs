mod events;
mod parser;
mod commands;
mod state;
mod cmd_defs;

pub use state::AppState;

pub use std::io;
use crossterm::event::{self, EnableMouseCapture};
use ratatui::{style::Stylize, DefaultTerminal};

fn set_display(app_state: &mut AppState) -> String {
    app_state.curr_count_lines = 0;
    let mut display = String::new();
    let dir_input = format!("{} -> {}", app_state.curr_dir, app_state.curr_input);
    let iter = app_state.output.iter().chain(std::iter::once(&dir_input));
    for out in iter {
        let mut flag = false;
        let mut start: usize = 0;
        let mut end: usize = app_state.screen_area.width as usize;
        while out.len() > end || flag{
            display.push_str(&out[start..end]);
            app_state.curr_count_lines += 1;
            if flag {
                break;  
            }
            display.push_str("\n");
            start += app_state.screen_area.width as usize;
            end += app_state.screen_area.width as usize;
            if end >= out.len() {
                end = out.len();
                flag = true;
            }
            
        }
        if out.len() <= app_state.screen_area.width as usize {
            display.push_str(&out);
            app_state.curr_count_lines += 1;
        }
        if out != "\n" {
            display.push_str("\n");
        }
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

    crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

    loop {
        terminal.draw(|frame| {
            app_state.screen_area = frame.area();
            app_state.screen_area.height = app_state.screen_area.height - 2;
            app_state.screen_area.width = app_state.screen_area.width - 2;
            let display = set_display(&mut app_state);
            let block = ratatui::widgets::Paragraph::new(display)
                .block(ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title("Rusty shell"))
                .scroll(((set_offset(&mut app_state) - app_state.scroll), 0))
                .on_black();
            frame.render_widget(block, frame.area());
    
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
    let result = commands::execute_command(app_state, parser::parse(&app_state.curr_dir, &app_state.curr_input));
    match result {
        cmd_defs::Command::Err(err) => {
            app_state.output.extend(err.to_vector());
        }
        _ => {}
    }
    app_state.curr_input.clear();
}
