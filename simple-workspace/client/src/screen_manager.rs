use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};

mod tabone;
mod tabtwo;

use crate::Result;
use tabone::TabOne;
use tabtwo::TabTwo;

#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    SwitchTab(Tab),
    None,
}

#[derive(Debug, Default, PartialEq)]
pub enum Tab {
    #[default]
    One,
    Two,
}

#[derive(Debug)]
pub struct ScreenManager {
    current_tab: Tab,

    tab_one: TabOne,
    tab_two: TabTwo,
}

impl ScreenManager {
    pub fn new() -> Self {
        Self {
            current_tab: Tab::default(),
            tab_one: TabOne {},
            tab_two: TabTwo {},
        }
    }

    pub fn handle_key(&mut self, key: &KeyCode) -> Result<Option<bool>> {
        match key {
            KeyCode::Char('q') => Ok(Some(true)),
            KeyCode::Tab => {
                self.toggle_tab();
                Ok(None)
            }
            KeyCode::Char('1') => {
                self.current_tab = Tab::One;
                Ok(None)
            }
            KeyCode::Char('2') => {
                self.current_tab = Tab::Two;
                Ok(None)
            }
            key => match self.current_tab {
                Tab::One => {
                    self.tab_one.handle_key(key)?;
                    Ok(None)
                }
                Tab::Two => {
                    self.tab_two.handle_key(key)?;
                    Ok(None)
                }
            },
        }
    }

    fn toggle_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::One => Tab::Two,
            Tab::Two => Tab::One,
        }
    }
}

pub trait Renderable {
    fn draw(&mut self, size: Rect, frame: &mut Frame);
}

pub trait Eventful {
    fn handle_key(&mut self, key: &KeyCode) -> Result<Command>;
}

impl Renderable for ScreenManager {
    fn draw(&mut self, size: Rect, frame: &mut Frame) {
        match self.current_tab {
            Tab::One => self.tab_one.draw(size, frame),
            Tab::Two => self.tab_two.draw(size, frame),
        }
    }
}
