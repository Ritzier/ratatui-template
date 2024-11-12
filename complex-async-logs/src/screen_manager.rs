use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::{Event, Result};
use main::Main;

mod main;

enum ScreenEvent {
    Quit,
    None,
}

enum ScreenState {
    Main,
}

pub struct ScreenManager {
    screen_state: ScreenState,
    command_tx: UnboundedSender<Event>,
    main: Main,
}

/// Handle App key and UI
impl ScreenManager {
    pub async fn new(command_tx: UnboundedSender<Event>) -> Result<Self> {
        Ok(Self {
            screen_state: ScreenState::Main,
            command_tx,
            main: Main::new().await?,
        })
    }

    pub async fn handle_key(&mut self, keycode: KeyCode) -> Result<()> {
        let screen_event = match self.screen_state {
            ScreenState::Main => self.main.handle_key(keycode).await?,
        };

        match screen_event {
            ScreenEvent::Quit => self.command_tx.send(Event::Quit)?,
            ScreenEvent::None => {}
        }

        Ok(())
    }

    pub fn draw(&mut self, area: Rect, frame: &mut Frame<'_>) {
        match self.screen_state {
            ScreenState::Main => self.main.draw(area, frame),
        }
    }
}
