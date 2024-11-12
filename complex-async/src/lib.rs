mod app;
mod error;
mod screen_manager;

pub use app::{App, Event};
pub use screen_manager::ScreenManager;

pub type Result<T> = std::result::Result<T, error::Error>;
