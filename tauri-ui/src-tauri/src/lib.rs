mod hosts;
mod network;

use network::SpeedTestResult;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

/// Application state shared across commands
pub struct AppState {
    speed_test_results: Mutex<Vec<SpeedTestResult>>,
    has_optimized: Mutex<bool>,
}

#[derive(Serialize)]
pub struct StatusResponse {
    enabled: bool,
    has_permission: bool,
    has_optimized: bool,
}

#[derive(Serialize)]
pub struct OperationResult {
    success: bool,
    message: String,
}

/// Get current acceleration status
#[tauri::command]
fn get_status(state: State<AppState>) -> StatusResponse {
    let enabled = hosts::is_enabled().unwrap_or(false);
    let has_permission = hosts::check_permission().is_ok();
    let has_optimized = *state.has_optimized.lock().unwrap();
    
    StatusResponse {
        enabled,
        has_permission,
        has_optimized,
    }
}

/// Check if we have admin permission
#[tauri::command]
fn check_permission() -> bool {
    hosts::check_permission().is_ok()
}

/// Enable acceleration with default IPs
#[tauri::command]
fn enable_acceleration() -> OperationResult {
    match hosts::enable() {
        Ok(_) => OperationResult {
            success: true,
            message: "Acceleration enabled successfully".to_string(),
        },
        Err(e) => OperationResult {
            success: false,
            message: format!("Failed to enable: {}", e),
        },
    }
}

/// Enable acceleration with optimized IPs
#[tauri::command]
fn enable_optimized(state: State<AppState>) -> OperationResult {
    let has_opt = *state.has_optimized.lock().unwrap();
    
    let result = if has_opt {
        hosts::enable_optimized()
    } else {
        hosts::enable()
    };
    
    match result {
        Ok(_) => OperationResult {
            success: true,
            message: if has_opt {
                "Optimized acceleration enabled".to_string()
            } else {
                "Acceleration enabled (no optimization data)".to_string()
            },
        },
        Err(e) => OperationResult {
            success: false,
            message: format!("Failed to enable: {}", e),
        },
    }
}

/// Disable acceleration
#[tauri::command]
fn disable_acceleration() -> OperationResult {
    match hosts::disable() {
        Ok(_) => OperationResult {
            success: true,
            message: "Acceleration disabled successfully".to_string(),
        },
        Err(e) => OperationResult {
            success: false,
            message: format!("Failed to disable: {}", e),
        },
    }
}

/// Run speed test and return results
#[tauri::command]
fn run_speed_test(state: State<AppState>) -> Vec<SpeedTestResult> {
    let raw_results = network::test_all_domains_parallel();
    
    // Convert to HashMap for hosts module
    let results_map: HashMap<String, (String, u64)> = raw_results.clone();
    hosts::set_optimized_ips(results_map);
    
    // Convert to display format
    let display_results = network::results_to_display(&raw_results);
    
    // Update state
    *state.speed_test_results.lock().unwrap() = display_results.clone();
    *state.has_optimized.lock().unwrap() = true;
    
    display_results
}

/// Get cached speed test results
#[tauri::command]
fn get_speed_test_results(state: State<AppState>) -> Vec<SpeedTestResult> {
    state.speed_test_results.lock().unwrap().clone()
}

/// Flush DNS cache (Windows only)
#[tauri::command]
fn flush_dns() -> OperationResult {
    #[cfg(target_os = "windows")]
    {
        match std::process::Command::new("ipconfig")
            .arg("/flushdns")
            .output()
        {
            Ok(_) => OperationResult {
                success: true,
                message: "DNS cache flushed successfully".to_string(),
            },
            Err(e) => OperationResult {
                success: false,
                message: format!("Failed to flush DNS: {}", e),
            },
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    OperationResult {
        success: false,
        message: "DNS flush not supported on this platform".to_string(),
    }
}

/// Open hosts file folder
#[tauri::command]
fn open_hosts_folder() -> OperationResult {
    #[cfg(target_os = "windows")]
    {
        match std::process::Command::new("explorer")
            .arg(r"C:\Windows\System32\drivers\etc")
            .spawn()
        {
            Ok(_) => OperationResult {
                success: true,
                message: "Opened hosts folder".to_string(),
            },
            Err(e) => OperationResult {
                success: false,
                message: format!("Failed to open folder: {}", e),
            },
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    OperationResult {
        success: false,
        message: "Not supported on this platform".to_string(),
    }
}

/// Open GitHub website
#[tauri::command]
fn open_github() -> OperationResult {
    #[cfg(target_os = "windows")]
    {
        match std::process::Command::new("cmd")
            .args(&["/C", "start", "https://github.com"])
            .spawn()
        {
            Ok(_) => OperationResult {
                success: true,
                message: "Opening GitHub...".to_string(),
            },
            Err(e) => OperationResult {
                success: false,
                message: format!("Failed to open GitHub: {}", e),
            },
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        match std::process::Command::new("xdg-open")
            .arg("https://github.com")
            .spawn()
        {
            Ok(_) => OperationResult {
                success: true,
                message: "Opening GitHub...".to_string(),
            },
            Err(e) => OperationResult {
                success: false,
                message: format!("Failed to open GitHub: {}", e),
            },
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        match std::process::Command::new("open")
            .arg("https://github.com")
            .spawn()
        {
            Ok(_) => OperationResult {
                success: true,
                message: "Opening GitHub...".to_string(),
            },
            Err(e) => OperationResult {
                success: false,
                message: format!("Failed to open GitHub: {}", e),
            },
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            speed_test_results: Mutex::new(Vec::new()),
            has_optimized: Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            get_status,
            check_permission,
            enable_acceleration,
            enable_optimized,
            disable_acceleration,
            run_speed_test,
            get_speed_test_results,
            flush_dns,
            open_hosts_folder,
            open_github,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
