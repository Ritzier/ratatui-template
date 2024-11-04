use crossterm::event::KeyCode;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use super::{Command, Eventful, Renderable, Tab};

#[derive(Debug)]
pub struct TabOne {}

impl Renderable for TabOne {
    fn draw(&mut self, size: Rect, frame: &mut Frame) {
        let paragraph = Paragraph::new("tab one");

        frame.render_widget(paragraph, size);
    }
}

impl Eventful for TabOne {
    fn handle_key(&mut self, key: &KeyCode) -> crate::Result<Command> {
        match key {
            KeyCode::Char('q') => Ok(Command::Quit),
            KeyCode::Tab => Ok(Command::SwitchTab(Tab::Two)),
            _ => Ok(Command::None),
        }
    }
}
