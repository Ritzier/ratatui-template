use std::{io::Stdout, time::Duration};

use crossterm::event::{
    Event as CrosstermEvent, EventStream as CrosstermEventStream, KeyCode, KeyEventKind,
};
use futures::{FutureExt, StreamExt};
use ratatui::{prelude::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{error::Error, Result, ScreenManager};

pub enum Event {
    Key(KeyCode),
    Tick,
    Frame,
    Quit,
}

pub struct App {
    should_quit: bool,
    crossterm_event: CrosstermEventStream,
    frame_rate: f64,
    tick_rate: f64,
    event_tx: UnboundedSender<Event>,
    event_rx: UnboundedReceiver<Event>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    screen_manager: ScreenManager,
}

impl App {
    pub async fn new(frame_rate: f64, tick_rate: f64) -> Result<Self> {
        let (event_tx, event_rx) = unbounded_channel();
        let screen_manager = ScreenManager::new(event_tx.clone()).await?;
        let terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

        Ok(Self {
            should_quit: false,
            crossterm_event: CrosstermEventStream::new(),
            frame_rate,
            tick_rate,
            event_tx,
            event_rx,
            screen_manager,
            terminal,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        startup()?;

        //let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

        let frame_rate = Duration::from_secs_f64(1.0 / self.frame_rate);
        let tick_rate = Duration::from_secs_f64(1.0 / self.tick_rate);
        let mut frame_interval = tokio::time::interval(frame_rate);
        let mut tick_interval = tokio::time::interval(tick_rate);

        while !self.should_quit {
            tokio::select! {
                _tick = tick_interval.tick() => {
                    self.event_tx.send(Event::Tick)?;
                }
                _frame = frame_interval.tick() => {
                    self.event_tx.send(Event::Frame)?;
                }
                event = self.crossterm_event.next().fuse() => {
                    match event.ok_or(Error::CrosstermEvent)?? {
                        CrosstermEvent::Key(key) => {
                            if let KeyEventKind::Press = key.kind {
                                self.event_tx.send(Event::Key(key.code))?;
                            }
                        }
                        _ => {}
                    }
                }
                Some(event) = self.event_rx.recv() => {
                    self.handle_event(event).await?;
                }

            }
        }

        shutdown()
    }

    async fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key) => self.screen_manager.handle_key(key).await?,
            Event::Frame => {
                self.terminal.draw(|frame| {
                    self.screen_manager.draw(frame.area(), frame);
                })?;
            }
            Event::Quit => self.should_quit = true,
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
