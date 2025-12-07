//! Hosts file manipulation module for GitHub acceleration
//!
//! Provides functionality to modify system hosts file for GitHub IP optimization

use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::path::Path;
use std::sync::{OnceLock, Mutex};
use std::collections::HashMap;

use crate::network;

const HOSTS_PATH_WINDOWS: &str = r"C:\Windows\System32\drivers\etc\hosts";
const HOSTS_PATH_UNIX: &str = "/etc/hosts";
const MARKER_START: &str = "# === FREE_TO_GITHUB START ===";
const MARKER_END: &str = "# === FREE_TO_GITHUB END ===";

// Default GitHub domain and IP mapping (fallback when no speed test)
const DEFAULT_GITHUB_HOSTS: &[(&str, &str)] = &[
    ("140.82.113.4", "github.com"),
    ("140.82.113.6", "api.github.com"),
    ("140.82.114.4", "gist.github.com"),
    ("185.199.108.153", "assets-cdn.github.com"),
    ("199.232.69.194", "github.global.ssl.fastly.net"),
    ("185.199.108.133", "raw.githubusercontent.com"),
    ("185.199.108.154", "github.githubassets.com"),
    ("140.82.113.10", "codeload.github.com"),
    ("185.199.108.133", "cloud.githubusercontent.com"),
    ("185.199.108.133", "avatars.githubusercontent.com"),
    ("185.199.108.133", "objects.githubusercontent.com"),
    ("185.199.108.153", "githubstatus.com"),
    ("140.82.113.22", "collector.github.com"),
];

// Global cache for optimized IPs (domain -> best_ip)
static OPTIMIZED_IPS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn get_optimized_ips() -> &'static Mutex<HashMap<String, String>> {
    OPTIMIZED_IPS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Build hosts content using optimized IPs if available, otherwise use defaults
fn build_hosts_content() -> Vec<u8> {
    let mut content = Vec::with_capacity(1024);
    writeln!(content, "\n{}", MARKER_START).unwrap();
    writeln!(content, "# Auto-optimized by Free to GitHub").unwrap();
    
    let optimized = get_optimized_ips().lock().unwrap();
    
    for entry in network::get_domain_candidates() {
        let ip = optimized
            .get(&entry.domain)
            .cloned()
            .unwrap_or_else(|| {
                entry.candidate_ips.first().cloned().unwrap_or_default()
            });
        
        if !ip.is_empty() {
            writeln!(content, "{} {}", ip, entry.domain).unwrap();
        }
    }
    
    writeln!(content, "{}", MARKER_END).unwrap();
    content
}

/// Build hosts content using default IPs (no speed test)
fn build_default_hosts_content() -> Vec<u8> {
    let mut content = Vec::with_capacity(512);
    writeln!(content, "\n{}", MARKER_START).unwrap();
    
    for (ip, domain) in DEFAULT_GITHUB_HOSTS {
        writeln!(content, "{} {}", ip, domain).unwrap();
    }
    
    writeln!(content, "{}", MARKER_END).unwrap();
    content
}

/// Update optimized IPs from speed test results
pub fn set_optimized_ips(results: HashMap<String, (String, u64)>) {
    let mut optimized = get_optimized_ips().lock().unwrap();
    optimized.clear();
    
    for (domain, (ip, _latency)) in results {
        optimized.insert(domain, ip);
    }
}

/// Check if we have optimized IPs available
pub fn has_optimized_ips() -> bool {
    let optimized = get_optimized_ips().lock().unwrap();
    !optimized.is_empty()
}

/// Get current optimized IP for a domain (if any)
pub fn get_optimized_ip(domain: &str) -> Option<String> {
    let optimized = get_optimized_ips().lock().unwrap();
    optimized.get(domain).cloned()
}

/// Clear optimized IPs cache
pub fn clear_optimized_ips() {
    let mut optimized = get_optimized_ips().lock().unwrap();
    optimized.clear();
}

pub fn get_hosts_path() -> &'static str {
    if cfg!(target_os = "windows") {
        HOSTS_PATH_WINDOWS
    } else {
        HOSTS_PATH_UNIX
    }
}

/// Check if GitHub acceleration is enabled
pub fn is_enabled() -> io::Result<bool> {
    let path = get_hosts_path();
    match fs::read_to_string(path) {
        Ok(content) => Ok(content.find(MARKER_START).is_some()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

/// Enable with default IPs (fast, no speed test)
pub fn enable() -> io::Result<()> {
    enable_with_ips(false)
}

/// Enable with optimized IPs (uses speed test results if available)
pub fn enable_optimized() -> io::Result<()> {
    enable_with_ips(true)
}

/// Internal enable function
fn enable_with_ips(use_optimized: bool) -> io::Result<()> {
    let hosts_path = get_hosts_path();
    
    let content = fs::read_to_string(hosts_path)?;
    
    // If already enabled, disable first to update IPs
    if content.find(MARKER_START).is_some() {
        disable()?;
    }
    
    let hosts_content = if use_optimized && has_optimized_ips() {
        build_hosts_content()
    } else {
        build_default_hosts_content()
    };
    
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(hosts_path)?;
    let mut writer = BufWriter::with_capacity(1024, file);
    writer.write_all(&hosts_content)?;
    writer.flush()?;

    Ok(())
}

/// Disable GitHub acceleration
pub fn disable() -> io::Result<()> {
    let hosts_path = get_hosts_path();
    let content = fs::read_to_string(hosts_path)?;
    
    match (content.find(MARKER_START), content.find(MARKER_END)) {
        (Some(start_pos), Some(end)) => {
            let end_pos = end + MARKER_END.len();
            
            let before = content[..start_pos].trim_end_matches('\n');
            let after = content[end_pos..].trim_start_matches('\n');
            
            let capacity = before.len() + after.len() + 2;
            let mut new_content = String::with_capacity(capacity);
            new_content.push_str(before);
            
            if !after.is_empty() {
                new_content.push('\n');
                new_content.push_str(after);
            }
            
            fs::write(hosts_path, new_content.into_bytes())?;
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Check if we have permission to modify hosts file
pub fn check_permission() -> Result<(), String> {
    let hosts_path = get_hosts_path();
    if !Path::new(hosts_path).exists() {
        return Err(format!("hosts file not found: {}", hosts_path));
    }

    match OpenOptions::new().append(true).open(hosts_path) {
        Ok(_) => Ok(()),
        Err(_) => {
            if cfg!(target_os = "windows") {
                Err("No permission to modify hosts file! Please run as Administrator".to_string())
            } else {
                Err("No permission to modify hosts file! Please run with sudo".to_string())
            }
        }
    }
}
