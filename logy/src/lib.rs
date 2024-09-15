mod log;
mod log_controller;
mod log_level;
mod mock;

pub use log_controller::*;
pub use log_level::*;
pub use mock::*;

use crate::log::Log;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref LOGGER: Mutex<Option<Arc<Mutex<Log<std::io::Stdout>>>>> = Mutex::new(None);
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
