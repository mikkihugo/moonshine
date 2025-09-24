//! Logging abstractions for Moon PDK integrations.
//!
//! Provides simple macros that proxy to Moon's host logging when running
//! inside the WASM extension, while falling back to the standard `log`
//! crate during native testing.

/// Supported log levels for Moon host logging.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

/// Dispatch a log message to the appropriate backend depending on the
/// compilation target. When the `wasm` feature is enabled we use the
/// Moon PDK host logging macros; otherwise we forward to the `log` crate
/// so tests can observe the messages if a logger is installed.
pub fn log_message(level: LogLevel, message: impl Into<String>) {
    let message = message.into();

    #[cfg(feature = "wasm")]
    {
        match level {
            LogLevel::Info => extism_pdk::info!("{}", message),
            LogLevel::Warn => extism_pdk::warn!("{}", message),
            LogLevel::Error => extism_pdk::error!("{}", message),
            LogLevel::Debug => extism_pdk::debug!("{}", message),
        }
    }

    #[cfg(not(feature = "wasm"))]
    {
        match level {
            LogLevel::Info => log::info!("{}", message),
            LogLevel::Warn => log::warn!("{}", message),
            LogLevel::Error => log::error!("{}", message),
            LogLevel::Debug => log::debug!("{}", message),
        }
    }
}

macro_rules! moon_info {
    ($($arg:tt)*) => {{
        $crate::moon_log::log_message($crate::moon_log::LogLevel::Info, format!($($arg)*));
    }};
}

macro_rules! moon_warn {
    ($($arg:tt)*) => {{
        $crate::moon_log::log_message($crate::moon_log::LogLevel::Warn, format!($($arg)*));
    }};
}

macro_rules! moon_error {
    ($($arg:tt)*) => {{
        $crate::moon_log::log_message($crate::moon_log::LogLevel::Error, format!($($arg)*));
    }};
}

macro_rules! moon_debug {
    ($($arg:tt)*) => {{
        $crate::moon_log::log_message($crate::moon_log::LogLevel::Debug, format!($($arg)*));
    }};
}
