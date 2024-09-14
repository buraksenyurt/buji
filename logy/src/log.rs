use crate::log_level::LogLevel;
use std::io::Write;

/// The main logger object that writes log messages to the specified target.
///
/// The `Log` object uses the `Write` trait to send logs to a target(example: `stdout` or a file)
///
/// # Generic Type Parameter
///
/// * `W` - An `Write` trait implementation which is where the log messages will be written.
pub struct Log<W: Write> {
    target: W,
}
impl<W: Write> Log<W> {
    /// Creates a new Log object.
    ///
    /// # Arguments
    ///
    /// * `target` - The target object where the log messages will be written to.
    ///
    /// # Returns
    ///
    /// A new instance of the `Log` object.
    pub fn new(target: W) -> Log<W> {
        Self { target }
    }

    /// Writes a log message to the target with the specified log level.
    /// Target is Write trait implementation. Something like that `std::io::stdout`
    ///
    /// # Arguments
    ///
    /// * `log_level` - The level of the log message (e.g., `Error`, `Warn`, `Info`).
    /// * `message` - The actual log message to be written.
    ///
    /// # Errors
    ///
    /// If there is an error writing to the target, an error message will be printed to `stderr`.
    pub fn write(&mut self, log_level: LogLevel, message: &str) {
        let color = log_level.to_ansi_color();
        let reset_color = "\x1b[0m";
        let entry = match log_level {
            LogLevel::Error => format!("{}[ERROR]: {}{}\n", color, message, reset_color),
            LogLevel::Warn => format!("{}[WARN] : {}{}\n", color, message, reset_color),
            LogLevel::Info => format!("{}[INFO] : {}{}\n", color, message, reset_color),
        };
        if let Err(e) = self.target.write_all(entry.as_bytes()) {
            eprintln!("Failed to write log : {}", e);
        }
    }
}
