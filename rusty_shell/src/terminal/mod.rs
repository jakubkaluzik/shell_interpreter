mod events;
mod parser;
mod commands;
mod state;
mod cmd_defs;

pub use state::AppState;

pub use std::io;
use crossterm::event::{self, EnableMouseCapture};
use ratatui::widgets::Wrap;
use ratatui::{style::Stylize,DefaultTerminal};


fn set_display(app_state: &mut AppState) -> String {
    app_state.screen_area.width -= 1;
    app_state.curr_count_lines = 0;
    for out in app_state.output.iter() {
        if out.len() > app_state.screen_area.width as usize {
            app_state.curr_count_lines += (out.len() as u16 + app_state.screen_area.width - 1) / app_state.screen_area.width;
        }
        else {
            app_state.curr_count_lines += 1;
        }
    }
    let mut display = app_state.output.join("\n");
    if display != "" {
        display.push_str("\n");
    }
    let dir_input = format!("{} -> {}¯", app_state.curr_dir, app_state.curr_input);
    
    if dir_input.len() > app_state.screen_area.width as usize {
        app_state.curr_count_lines += (dir_input.len() as u16 + app_state.screen_area.width - 1) / app_state.screen_area.width;
    }
    display.push_str(dir_input.as_str());
    display
}
//TODO handle cases where the input and the saved input is too long that it wraps, current logic does not
//handle this case because the wrapping does not cout as line break \n
fn set_offset(app_state: &mut AppState) -> u16 {
    app_state.screen_area.height -= 3;
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
            let display = set_display(&mut app_state);
            let block = ratatui::widgets::Paragraph::new(display)
                .block(ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title("Rusty shell"))
                .wrap(Wrap { trim: false })
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
