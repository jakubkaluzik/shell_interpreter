use ratatui::prelude::Rect;
use std::collections::VecDeque;
use std::env;

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

pub struct CursorState {
    pub x: u16,
    pub y: u16,
}
impl CursorState {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}