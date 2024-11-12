use crossterm::event::KeyCode;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::Result;

use super::ScreenEvent;

pub struct Main {}

impl Main {
    pub async fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub async fn handle_key(&mut self, key: KeyCode) -> Result<ScreenEvent> {
        match key {
            KeyCode::Char('q') => Ok(ScreenEvent::Quit),
            _ => Ok(ScreenEvent::None),
        }
    }

    pub fn draw(&self, area: Rect, frame: &mut Frame<'_>) {
        let paragraph = Paragraph::new("Hi");
        frame.render_widget(paragraph, area);
    }
}
