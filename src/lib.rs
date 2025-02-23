//! A utility crate for enhancing `Result<T, E>` with logging capabilities.
//!
//! This crate provides the `ResultLogging` trait, which extends `Result<T, E>` with methods to
//! log errors at different log levels using the `log` crate.
//!
//! # Example Usage
//! ```
//! use log::{LevelFilter, info};
//! use simple_logger::SimpleLogger;
//! use result_logger::ResultLogger;
//!
//! fn example_function() -> Result<(), &'static str> {
//!     Err("Something went wrong!")
//! }
//!
//! fn main() {
//!     SimpleLogger::new().init().unwrap();
//!     let _ = example_function().error(); // Logs the error message at the error level
//! }
//! ```

use log::{debug, error, info, trace, warn};

/// A trait that provides logging capabilities for `Result<T, E>`.
///
/// This trait allows logging the `Err` variant of a `Result` at various log levels
/// without consuming the result, enabling error tracking without affecting the
/// control flow.
pub trait ResultLogger {
    /// Logs an error message at the TRACE level if the `Result` is an `Err`.
    fn trace(self) -> Self;
    /// Logs an error message at the DEBUG level if the `Result` is an `Err`.
    fn debug(self) -> Self;
    /// Logs an error message at the INFO level if the `Result` is an `Err`.
    fn info(self) -> Self;
    /// Logs an error message at the WARN level if the `Result` is an `Err`.
    fn warn(self) -> Self;
    /// Logs an error message at the ERROR level if the `Result` is an `Err`.
    fn error(self) -> Self;
}

impl<T, E: std::fmt::Display> ResultLogger for Result<T, E> {
    fn trace(self) -> Self {
        self.inspect_err(|e| trace!("{}", e))
    }

    fn debug(self) -> Self {
        self.inspect_err(|e| debug!("{}", e))
    }

    fn info(self) -> Self {
        self.inspect_err(|e| info!("{}", e))
    }

    fn warn(self) -> Self {
        self.inspect_err(|e| warn!("{}", e))
    }

    fn error(self) -> Self {
        self.inspect_err(|e| error!("{}", e))
    }
}

#[cfg(test)]
mod tests {}
