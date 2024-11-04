use std::{io::Stdout, time::Duration};

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyEventKind};
use futures::{FutureExt, StreamExt};
use ratatui::{prelude::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{
    screen_manager::{Renderable, ScreenManager},
    Error, Result,
};

#[derive(Debug)]
pub enum Event {
    Key(KeyCode),
    Tick,
    Render,
}

#[derive(Debug)]
pub struct App {
    should_quit: bool,
    crossterm_event: EventStream,
    frame_rate: f64,
    tick_rate: f64,
    event_rx: UnboundedReceiver<Event>,
    event_tx: UnboundedSender<Event>,
    screen_manager: ScreenManager,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    pub fn new(frame_rate: f64, tick_rate: f64) -> Result<Self> {
        let (event_tx, event_rx) = unbounded_channel();
        let crossterm_event = EventStream::new();
        let terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

        Ok(Self {
            should_quit: false,
            event_rx,
            event_tx,
            frame_rate,
            tick_rate,
            crossterm_event,
            terminal,
            screen_manager: ScreenManager::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        startup()?;

        let frame_rate = Duration::from_secs_f64(1.0 / self.frame_rate);
        let tick_rate = Duration::from_secs_f64(1.0 / self.tick_rate);
        let mut frame_interval = tokio::time::interval(frame_rate);
        let mut tick_interval = tokio::time::interval(tick_rate);

        while !self.should_quit {
            tokio::select! {

                _tick = tick_interval.tick() => {
                    self.event_tx.send(Event::Tick).unwrap();
                }

                _frame = frame_interval.tick() => {
                    self.event_tx.send(Event::Render).unwrap();
                }

                Some(event) = self.event_rx.recv() => {
                    match event {
                        Event::Render => {
                            self.terminal.draw(|frame| {
                                self.screen_manager.draw(frame.area(),frame);
                            })?;
                        }
                        Event::Key(key) => {
                            if let Ok(Some(true)) = self.screen_manager.handle_key(&key) {
                                self.should_quit=true
                            }
                        }
                        _ => {}
                    }
                }

                event = self.crossterm_event.next().fuse() => {
                    match event.ok_or(Error::Crossterm)?? {
                        CrosstermEvent::Key(key) => {
                            if let KeyEventKind::Press = key.kind {
                                self.event_tx.send(Event::Key(key.code))?
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        shutdown()?;
        Ok(())
    }
}

fn startup() -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
