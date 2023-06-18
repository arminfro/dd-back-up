use env_logger::fmt::Color;
use env_logger::{Builder, Env};
use log::Level;
use std::io::Write;

/// Configures the logger with the desired log level and format.
///
/// The log level can be adjusted by setting the `RUST_LOG` environment variable.
/// Valid log levels are `trace`, `debug`, `info`, `warn`, and `error`.
/// If the `RUST_LOG` environment variable is not set, the default log level is `info`.
///
/// The logger format includes the timestamp, log level, target module, and log message.
/// Log levels are color-coded for better readability.
pub fn configure_logger() {
    Builder::from_env(Env::default().filter_or("RUST_LOG", "info"))
        .format(|buf, record| {
            let level = record.level();
            let level_color = match level {
                Level::Trace => Color::White,
                Level::Debug => Color::Blue,
                Level::Info => Color::Green,
                Level::Warn => Color::Yellow,
                Level::Error => Color::Red,
            };

            let mut level_style = buf.style();
            level_style.set_color(level_color).set_bold(true);
            let level_str = match level {
                log::Level::Trace => "TRACE",
                log::Level::Debug => "DEBUG",
                log::Level::Info => "INFO ",
                log::Level::Warn => "WARN ",
                log::Level::Error => "ERROR",
            };

            let target = record.target();

            let mut target_style = buf.style();
            target_style
                .set_color(Color::Rgb(255, 165, 0))
                .set_bold(false);

            writeln!(
                buf,
                "[{} {} - {}]: {}",
                buf.timestamp(),
                level_style.value(level_str),
                target_style.value(target),
                record.args()
            )
        })
        .init();
}
