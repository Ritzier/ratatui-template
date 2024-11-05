use std::{io::Stdout, time::Duration};

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyEventKind};
use futures::{FutureExt, StreamExt};
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph, Terminal};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{Error, Result};

#[derive(Debug)]
pub enum Event {
    Key(KeyCode),
    Tick,
    Frame,
}

#[derive(Debug)]
pub struct App {
    should_quit: bool,
    crossterm_event: EventStream,
    frame_rate: f64,
    tick_rate: f64,
    event_rx: UnboundedReceiver<Event>,
    event_tx: UnboundedSender<Event>,
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
                    self.event_tx.send(Event::Frame).unwrap();
                }
                Some(event) = self.event_rx.recv() => {
                    self.handle_event(&event)?;
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

    fn handle_event(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::Key(key) => self.handle_key_event(key)?,
            Event::Frame => {
                self.terminal.draw(|frame| {
                    frame.render_widget(Paragraph::new("Hi"), frame.area());
                })?;
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, keycode: &KeyCode) -> Result<()> {
        match keycode {
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }

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
