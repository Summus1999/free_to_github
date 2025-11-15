pub mod hosts;
pub mod logger;

/// Logging macros exported for all binaries to use
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log::warn!($($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log::error!($($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*)
    };
}
