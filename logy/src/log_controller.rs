use crate::log::Log;
use crate::LOGGER;
use std::sync::{Arc, Mutex};

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
