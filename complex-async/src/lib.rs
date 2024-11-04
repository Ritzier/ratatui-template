mod app;
mod error;
mod screen_manager;

use error::Error;

pub use app::App;

pub type Result<T> = std::result::Result<T, Error>;
