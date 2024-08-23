use std::io::Write;

/// Enum representing the log states.
pub enum LogLevel {
    /// Error state, shows there is an error
    Error,
    /// Warning states, shows there is a warning situation
    Warn,
    /// Information state, use for an information purposes
    Info,
}

/// Main logger object which is use Write Trait implementation.
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
    /// # Example
    ///
    /// ```
    /// use std::io;
    /// use buji::{Log,LogLevel};
    ///
    /// let stdout = io::stdout();
    /// let mut logger = Log::new(stdout);
    ///
    /// logger.write(LogLevel::Info, "This is an info message");
    /// logger.write(LogLevel::Warn, "This is a warning message");
    /// logger.write(LogLevel::Error, "This is an error message");
    /// ```
    ///
    /// Output of the preceding codes:
    ///
    /// ```text
    /// [INFO] : This is an info message
    /// [WARN] : This is a warning message
    /// [ERROR]: This is an error message
    /// ```
    ///
    /// # Errors
    ///
    /// If there is an error writing to the target, an error message will be printed to `stderr`.
    pub fn write(&mut self, log_level: LogLevel, message: &str) {
        let entry = match log_level {
            LogLevel::Error => format!("[ERROR]: {}\n", message),
            LogLevel::Warn => format!("[WARN] : {}\n", message),
            LogLevel::Info => format!("[INFO] : {}\n", message),
        };
        if let Err(e) = self.target.write_all(entry.as_bytes()) {
            eprintln!("Failed to write log : {}", e);
        }
    }
}