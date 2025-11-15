#[cfg(debug_assertions)]
use std::fs::OpenOptions;
use std::io::Write;
#[cfg(debug_assertions)]
use std::path::PathBuf;
use std::sync::Mutex;
use log::{Level, Metadata, Record};

/// Simple file logger implementation for performance diagnostics
pub struct FileLogger {
    file: Mutex<Option<std::fs::File>>,
}

impl FileLogger {
    /// Initialize the logger with a log file (only in debug builds)
    /// In release builds, this function does nothing
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(debug_assertions)]
        {
            let log_path = Self::get_log_path()?;
            
            // Ensure the log directory exists
            if let Some(parent) = log_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            // Open or create the log file in append mode
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)?;
            
            let logger = Box::leak(Box::new(FileLogger {
                file: Mutex::new(Some(file)),
            }));
            
            log::set_logger(logger)
                .map(|()| log::set_max_level(log::LevelFilter::Info))?;
            
            log::info!("Logger initialized at {}", log_path.display());
        }
        
        #[cfg(not(debug_assertions))]
        {
            // No-op for release builds - logging disabled
        }
        
        Ok(())
    }
    
    /// Get the log file path based on the operating system
    #[cfg(debug_assertions)]
    fn get_log_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let log_dir = if cfg!(target_os = "windows") {
            // Use %APPDATA% on Windows
            match std::env::var("APPDATA") {
                Ok(appdata) => PathBuf::from(appdata).join("free_to_github"),
                Err(_) => PathBuf::from(".").join(".free_to_github"),
            }
        } else {
            // Use ~/.local/share on Unix-like systems
            dirs_home()
                .join(".local/share/free_to_github")
        };
        
        Ok(log_dir.join("connection.log"))
    }
    
    /// Write a formatted log entry to file
    #[cfg(debug_assertions)]
    fn write_log(&self, message: &str) -> std::io::Result<()> {
        if let Ok(mut file_opt) = self.file.lock() {
            if let Some(ref mut file) = *file_opt {
                writeln!(file, "{}", message)?;
                file.flush()?;
            }
        }
        Ok(())
    }
    
    /// No-op write_log for release builds
    #[cfg(not(debug_assertions))]
    fn write_log(&self, _message: &str) -> std::io::Result<()> {
        Ok(())
    }
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        
        let now = chrono_time();
        let formatted = format!(
            "[{}] [{}] {}",
            now,
            record.level(),
            record.args()
        );
        
        let _ = self.write_log(&formatted);
    }
    
    fn flush(&self) {
        if let Ok(mut file_opt) = self.file.lock() {
            if let Some(ref mut file) = *file_opt {
                let _ = file.flush();
            }
        }
    }
}

/// Get the current timestamp as a formatted string (HH:MM:SS.mmm)
fn chrono_time() -> String {
    use std::time::SystemTime;
    
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    
    // Convert to human-readable time format
    let total_secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    // Calculate hours, minutes, seconds from Unix timestamp
    // Note: This is a simplified calculation in UTC timezone
    let secs_today = total_secs % 86400;  // Seconds in a day
    let hours = secs_today / 3600;
    let minutes = (secs_today % 3600) / 60;
    let seconds = secs_today % 60;
    
    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

/// Get home directory path (cross-platform)
#[cfg(debug_assertions)]
fn dirs_home() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERPROFILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    }
}

/// Log connection performance metrics (only recorded in debug builds)
pub fn log_connection_metrics(
    _target: &str,
    _duration_ms: u128,
    _success: bool,
) {
    #[cfg(debug_assertions)]
    {
        if _success {
            log::info!(
                "Connection to {} completed in {} ms ✓",
                _target, _duration_ms
            );
        } else {
            log::warn!(
                "Connection to {} failed or timed out (took {} ms)",
                _target, _duration_ms
            );
        }
    }
}

/// Log hosts file operation (only recorded in debug builds)
pub fn log_hosts_operation(_operation: &str, _duration_ms: u128, _success: bool) {
    #[cfg(debug_assertions)]
    {
        if _success {
            log::info!(
                "Hosts {} operation completed in {} ms ✓",
                _operation, _duration_ms
            );
        } else {
            log::error!(
                "Hosts {} operation failed (took {} ms)",
                _operation, _duration_ms
            );
        }
    }
}

