//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use documentaion_lab::config::Logging;
//! let config = Logging::new(Logging::DEFAULT_DESTINATION);
//! ```
//!
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use documentaion_lab::config::Logging;
/// let config = Logging::new(Logging::DEFAULT_DESTINATION);
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    destination: LogOutput,
}

impl Logging {
    pub fn new(destination: LogOutput) -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination,
        }
    }

    /// Method for changing the output for logger
    /// # Examples:
    /// ```
    /// use documentaion_lab::config::{Logging, LogLevel, LogOutput};
    /// let mut config = Logging::new(Logging::DEFAULT_DESTINATION);
    /// let output = config.destination();
    /// config.set_destination(LogOutput::Stderr)
    /// ```
    pub fn set_destination(&mut self, out: LogOutput) {
        self.destination = out;
    }

    pub fn destination(&self) -> &LogOutput {
        &self.destination
    }

    pub const DEFAULT_DESTINATION: LogOutput = LogOutput::Stdout;
}
