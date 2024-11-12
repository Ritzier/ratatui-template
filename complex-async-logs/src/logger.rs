use time::macros::format_description;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    fmt::{self, time::UtcTime},
    prelude::*,
    Layer,
};

pub fn logger_init() {
    // Setup log file
    let file_appender = RollingFileAppender::new(Rotation::MINUTELY, "logs", "tiktok-tui.log");

    // Create file layer
    let file_layer = fmt::Layer::new()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_timer(UtcTime::new(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        )))
        .with_writer(file_appender)
        .with_ansi(false);

    tracing_subscriber::registry()
        .with(file_layer.with_filter(LevelFilter::DEBUG))
        .init();
}
