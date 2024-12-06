use crossterm::event::{self, EnableMouseCapture};
use ratatui::widgets::Wrap;
use ratatui::{style::Stylize,DefaultTerminal};
use ratatui::prelude::Rect;
use std::collections::VecDeque;
use std::env;

pub use std::io;
mod events;

pub struct AppState {
    pub curr_input: String,
    pub prev_inputs: VecDeque<String>,
    pub output: Vec<String>,
    pub curr_dir: String,
    pub curr_prev_input: usize,
    pub max_prev_inputs: usize,
    pub is_displayed: bool,
    pub scroll: u16,
    pub screen_area: Rect,
    pub curr_count_lines: u16,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            curr_input: String::new(),
            prev_inputs: VecDeque::new(),
            output: Vec::new(),
            curr_dir: env::current_dir().unwrap().to_str().unwrap().to_string(),
            curr_prev_input: 0,
            max_prev_inputs: 10,
            is_displayed: false,
            scroll: 0,
            screen_area: Rect::new(0, 0, 0, 0),
            curr_count_lines: 0,
        }
    }
}
fn set_display(app_state: &mut AppState) -> String {
    let mut display = app_state.output.join("\n");
    if display != "" {
        display.push_str("\n");
     }
    display.push_str(&format!("{} -> {}¯", app_state.curr_dir, app_state.curr_input));
    display
}
//TODO handle cases where the input and the saved input is too long that it wraps, current logic does not
//handle this case because the wrapping does not cout as line break \n
fn set_offset(app_state: &mut AppState) -> u16 {
    app_state.screen_area.height -= 3;
    app_state.screen_area.width -= 3;
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
            app_state.curr_count_lines = display.matches('\n').count() as u16;
            let block = ratatui::widgets::Paragraph::new(display)
                .block(ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title("Rusty shell"))
                .wrap(Wrap { trim: true })
                .scroll(((set_offset(&mut app_state) - app_state.scroll), 0))
                .on_black();
            frame.render_widget(block, frame.area());
    
        })?;

        
        if let Ok(event) = event::read() {
            if let events::EventResult::Exit = events::handle_event(event, &mut app_state) {
                break;
            }
        }
    }
    Ok(())
}
