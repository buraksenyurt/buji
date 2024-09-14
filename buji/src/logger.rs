use lazy_static::lazy_static;
use std::io::Write;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref LOGGER: Mutex<Option<Arc<Mutex<Log<std::io::Stdout>>>>> = Mutex::new(None);
}

/// A global logger controller that manages logger.
///
/// This object ensures that the logger can be globally accessed across the application.
pub struct LogController;

impl LogController {
    /// Sets the global logger instance.
    pub fn init_logger() {
        let logger = Arc::new(Mutex::new(Log::new(std::io::stdout())));
        let mut global_logger = LOGGER.lock().unwrap();
        *global_logger = Some(logger);
    }

    /// Sets the global logger instance.
    ///
    /// # Arguments
    ///
    /// * `logger` - The logger instance wrapped in `Arc` and `Mutex`.
    ///
    /// This function sets the logger to be used globally across the application.
    pub fn set_logger(logger: Arc<Mutex<Log<std::io::Stdout>>>) {
        let mut global_logger = LOGGER.lock().unwrap();
        *global_logger = Some(logger);
    }

    /// Retrieves the global logger instance if it has been set.
    ///
    /// # Returns
    ///
    /// An `Option` containing the logger wrapped in `Arc` and `Mutex`, or `None` if the logger is not set.
    pub fn get_logger() -> Option<Arc<Mutex<Log<std::io::Stdout>>>> {
        let global_logger = LOGGER.lock().unwrap();
        global_logger.clone()
    }
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

impl LogLevel {
    /// Converts a `LogLevel` to the corresponding ANSI color code.
    ///
    /// # Returns
    ///
    /// A string slice representing the ANSI color code for the log level.
    ///
    /// - `Error`: Red
    /// - `Warn`: Yellow
    /// - `Info`: Blue
    pub fn to_ansi_color(&self) -> &str {
        match self {
            LogLevel::Error => "\x1b[91m", // Red
            LogLevel::Warn => "\x1b[93m",  // Yellow
            LogLevel::Info => "\x1b[94m",  // Blue
        }
    }
}

/// A macro for logging messages using the provided logger if available.
///
/// This macro checks if the logger exists
/// If so writes the log message to the target.
/// If no logger is provided the macro does nothing.
///
/// # Arguments
///
/// * `$log_level`: The log level to use (`LogLevel::Error`, `LogLevel::Warn`, `LogLevel::Info`).
/// * `$message`: The message to log, as a string slice.
///
/// # Panics
///
/// This macro does not panic.
#[macro_export]
macro_rules! linfo {
    ($log_level:expr,$message:expr) => {
        if let Some(logger) = LogController::get_logger() {
            let mut logger_ref = logger.lock().unwrap();
            logger_ref.write($log_level, $message)
        }
    };
}
