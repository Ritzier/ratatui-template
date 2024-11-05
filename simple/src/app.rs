use ratatui::{
    crossterm::{
        self,
        event::{self, Event, KeyCode, KeyEventKind},
    },
    layout::Rect,
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

use crate::Result;

pub struct App {
    should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self { should_quit: false })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        startup()?;

        while !self.should_quit {
            terminal.draw(|frame| {
                self.render(frame.area(), frame);
            })?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                self.handle_key_event(&key.code)?;
            }
        }

        shutdown()
    }

    fn handle_key_event(&mut self, key: &KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            _ => {}
        }
        Ok(())
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        let paragraph = Paragraph::new("Hi");
        frame.render_widget(paragraph, area);
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
