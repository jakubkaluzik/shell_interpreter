use ratatui::crossterm::event::{self};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{style::Stylize,DefaultTerminal};
use std::env;

pub use std::io;
mod events;

pub struct AppState {
    pub curr_input: String,
    pub prev_inputs: Vec<String>,
    pub curr_dir: String,
    pub output: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            curr_input: String::new(),
            prev_inputs: Vec::new(),
            curr_dir: env::current_dir().unwrap().to_str().unwrap().to_string(),
            output: String::new(),
        }
    }
}

pub fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut app_state = AppState::new();

    loop {
        terminal.draw(|frame| {
            let display = app_state.curr_dir.clone() + ">" + &app_state.curr_input.clone();
            let block = ratatui::widgets::Paragraph::new(display)
                .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title("Rusty shell"))
                .wrap(Wrap { trim: true })
                .on_black();

            frame.render_widget(block, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if let events::EventResult::Exit = events::handle_events(key, &mut app_state) {
                break;
            }
        }
    }
    Ok(())
}
