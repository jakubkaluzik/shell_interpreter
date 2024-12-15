use ratatui::prelude::Rect;
use std::collections::VecDeque;
use std::env;

//Structure for data that user sees in the terminal
pub struct Display {
    pub curr_input: String,
    pub output: Vec<String>,
    pub curr_dir: String,
}
impl Display {
    pub fn new() -> Self {
        Self {
            curr_input: String::new(),
            output: Vec::new(),
            curr_dir: env::current_dir().unwrap().to_str().unwrap().to_string(),
        }
    }
}
//Structure for holding data about the user's input history
pub struct UserInputHistory {
    pub prev_inputs: VecDeque<String>,
    pub curr_prev_input: usize,
    pub max_prev_inputs: usize,
    pub is_displayed: bool,
}
impl UserInputHistory {
    pub fn new() -> Self {
        Self {
            prev_inputs: VecDeque::new(),
            curr_prev_input: 0,
            max_prev_inputs: 10,
            is_displayed: false,
        }
    }
}
//Structure used for storing data about the scroll state
pub struct Scroll {
    pub offset: u16,
    pub curr_scroll: u16,
    pub curr_count_lines: u16,
}
impl Scroll {
    pub fn new() -> Self {
        Self {
            offset: 0,
            curr_scroll: 0,
            curr_count_lines: 0,
        }
    }
}
//Structure for holding the state of the cursor
pub struct CursorState {
    pub x: u16,
    pub y: u16,
}
impl CursorState {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}
//Structure for holding the state of the application
pub struct AppState {
    pub display: Display,
    pub past: UserInputHistory,
    pub scroll: Scroll,
    pub cursor: CursorState,
    pub screen_area: Rect,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            display: Display::new(),
            past: UserInputHistory::new(),
            scroll: Scroll::new(),
            cursor: CursorState::new(),
            screen_area: Rect::new(0, 0, 0, 0),
        }
    }
}