use std::io::Write;

/// A macro for logging messages using the provided logger if available.
///
/// This macro checks if the logger exists
/// If so writes the log message to the target.
/// If no logger is provided the macro does nothing.
///
/// # Arguments
///
/// * `$logger`: The optional logger (`Option<Log<W>>`) where the log message should be written.
/// * `$log_level`: The log level to use (`LogLevel::Error`, `LogLevel::Warn`, `LogLevel::Info`).
/// * `$message`: The message to log, as a string slice.
///
/// # Panics
///
/// This macro does not panic.
#[macro_export]
macro_rules! linfo {
    ($logger:expr,$log_level:expr,$message:expr) => {
        if let Some(ref mut logger) = $logger {
            let mut logger_ref = logger.borrow_mut();
            logger_ref.write($log_level, $message);
        }
    };
}

/// Enum representing the log states.
#[derive(Clone)]
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
    /// use buji::{Log, LogLevel, MockLogger};
    ///
    /// let mut logger = Log::new(MockLogger);
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

/// Mock instance object of Log struct to use on tests.
pub struct MockLogger;

impl Write for MockLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
