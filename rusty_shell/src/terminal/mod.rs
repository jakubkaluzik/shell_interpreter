use crossterm::event::{self, EnableMouseCapture};
use ratatui::widgets::{List, ListItem, Wrap};
use ratatui::{style::Stylize,DefaultTerminal};
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
    pub scroll: usize,
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
        }
    }
}

pub fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut app_state = AppState::new();

    crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

    loop {
        terminal.show_cursor()?;
        terminal.draw(|frame| {
            let mut display = app_state.output.join("");
            display.push_str(&format!("{} -> {}", app_state.curr_dir, app_state.curr_input));

            let block = ratatui::widgets::Paragraph::new(display)
                .block(ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title("Rusty shell"))
                .wrap(Wrap { trim: true })
                .on_black();
            frame.render_widget(block, frame.area());
    
        })?;

        
        if let Ok(event) = event::read() {
            if let events::EventResult::Exit = events::handle_event(event, &mut app_state) {
                break;
            }
        }
        //if let event::Event::Mouse(mouse) = event::read()? {
        //    events::mouse_event(mouse, &mut app_state);
        //}
    }
    Ok(())
}
