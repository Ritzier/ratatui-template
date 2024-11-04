use crossterm::event::KeyCode;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use super::{Command, Eventful, Renderable, Tab};

#[derive(Debug)]
pub struct TabTwo {}

impl Renderable for TabTwo {
    fn draw(&mut self, size: Rect, frame: &mut Frame) {
        let paragraph = Paragraph::new("tab two");

        frame.render_widget(paragraph, size);
    }
}

impl Eventful for TabTwo {
    fn handle_key(&mut self, key: &KeyCode) -> crate::Result<Command> {
        match key {
            KeyCode::Char('q') => Ok(Command::Quit),
            KeyCode::Tab => Ok(Command::SwitchTab(Tab::One)),
            _ => Ok(Command::None),
        }
    }
}
