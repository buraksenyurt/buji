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
