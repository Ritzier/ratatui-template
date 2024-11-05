use app::App;

mod app;
mod error;

type Result<T> = std::result::Result<T, error::Error>;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    App::new()?.run(terminal)
}
