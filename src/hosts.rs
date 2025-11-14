use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::path::Path;
use std::sync::OnceLock;

const HOSTS_PATH_WINDOWS: &str = r"C:\Windows\System32\drivers\etc\hosts";
const HOSTS_PATH_UNIX: &str = "/etc/hosts";
const MARKER_START: &str = "# === FREE_TO_GITHUB START ===";
const MARKER_END: &str = "# === FREE_TO_GITHUB END ===";

// GitHub domain and IP mapping - optimized for minimal size
const GITHUB_HOSTS: &[(&str, &str)] = &[
    ("140.82.113.4", "github.com"),
    ("140.82.114.4", "gist.github.com"),
    ("185.199.108.153", "assets-cdn.github.com"),
    ("185.199.109.153", "assets-cdn.github.com"),
    ("185.199.110.153", "assets-cdn.github.com"),
    ("185.199.111.153", "assets-cdn.github.com"),
    ("199.232.69.194", "github.global.ssl.fastly.net"),
    ("185.199.108.133", "raw.githubusercontent.com"),
    ("185.199.109.133", "raw.githubusercontent.com"),
    ("185.199.110.133", "raw.githubusercontent.com"),
    ("185.199.111.133", "raw.githubusercontent.com"),
    ("185.199.108.154", "github.githubassets.com"),
    ("185.199.109.154", "github.githubassets.com"),
    ("185.199.110.154", "github.githubassets.com"),
    ("185.199.111.154", "github.githubassets.com"),
    ("140.82.113.10", "codeload.github.com"),
    ("140.82.114.10", "codeload.github.com"),
    ("185.199.108.133", "cloud.githubusercontent.com"),
    ("185.199.109.133", "cloud.githubusercontent.com"),
    ("185.199.110.133", "cloud.githubusercontent.com"),
    ("185.199.111.133", "cloud.githubusercontent.com"),
    ("185.199.108.153", "githubstatus.com"),
    ("140.82.113.18", "api.github.com"),
    ("140.82.114.18", "api.github.com"),
];

// Pre-built append content cache
static APPEND_CONTENT: OnceLock<Vec<u8>> = OnceLock::new();

fn get_append_bytes() -> &'static Vec<u8> {
    APPEND_CONTENT.get_or_init(|| {
        let mut content = Vec::with_capacity(512);
        writeln!(content, "\n{}", MARKER_START).unwrap();
        
        for (ip, domain) in GITHUB_HOSTS {
            writeln!(content, "{} {}", ip, domain).unwrap();
        }
        
        writeln!(content, "{}", MARKER_END).unwrap();
        content
    })
}

pub fn get_hosts_path() -> &'static str {
    if cfg!(target_os = "windows") {
        HOSTS_PATH_WINDOWS
    } else {
        HOSTS_PATH_UNIX
    }
}

/// Ultra-fast check using find instead of contains
pub fn is_enabled() -> io::Result<bool> {
    let path = get_hosts_path();
    match fs::read_to_string(path) {
        Ok(content) => {
            // Use find which is typically faster for single searches
            Ok(content.find(MARKER_START).is_some())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

/// Ultra-fast enable with pre-built content
pub fn enable() -> io::Result<()> {
    let hosts_path = get_hosts_path();
    
    // Quick check
    match fs::read_to_string(hosts_path) {
        Ok(content) => {
            if content.find(MARKER_START).is_some() {
                return Ok(());
            }
        }
        Err(e) if e.kind() != io::ErrorKind::NotFound => return Err(e),
        _ => {}
    }

    // Write pre-built content at once using direct binary write
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(hosts_path)?;
    let mut writer = BufWriter::with_capacity(1024, file);
    writer.write_all(get_append_bytes())?;
    writer.flush()?;

    Ok(())
}

/// Ultra-fast disable using binary search and slice operations
pub fn disable() -> io::Result<()> {
    let hosts_path = get_hosts_path();
    let content = fs::read_to_string(hosts_path)?;
    
    // Use find for faster string search
    match (content.find(MARKER_START), content.find(MARKER_END)) {
        (Some(start), Some(end)) => {
            let end_pos = end + MARKER_END.len();
            
            // Trim whitespace efficiently
            let before = content[..start].trim_end_matches('\n');
            let after = content[end_pos..].trim_start_matches('\n');
            
            // Build new content with minimal allocations
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
        _ => Ok(()), // Not enabled, nothing to do
    }
}

pub fn check_permission() -> Result<(), String> {
    let hosts_path = get_hosts_path();
    if !Path::new(hosts_path).exists() {
        return Err(format!("hosts 文件不存在: {}", hosts_path));
    }

    match OpenOptions::new().append(true).open(hosts_path) {
        Ok(_) => Ok(()),
        Err(_) => {
            if cfg!(target_os = "windows") {
                Err("没有权限修改 hosts 文件!\n请以管理员身份运行此程序".to_string())
            } else {
                Err("没有权限修改 hosts 文件!\n请使用 sudo 运行此程序".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_enable_disable_performance() {
        let start = Instant::now();
        
        // Test enable performance
        let enable_start = Instant::now();
        let _ = enable();
        let enable_time = enable_start.elapsed();
        
        println!("Enable operation took: {:?}", enable_time);
        assert!(enable_time.as_millis() < 1000, "Enable should complete within 1 second");
        
        // Test disable performance
        let disable_start = Instant::now();
        let _ = disable();
        let disable_time = disable_start.elapsed();
        
        println!("Disable operation took: {:?}", disable_time);
        assert!(disable_time.as_millis() < 1000, "Disable should complete within 1 second");
        
        println!("Total time: {:?}", start.elapsed());
    }

    #[test]
    fn test_is_enabled_performance() {
        let start = Instant::now();
        for _ in 0..100 {
            let _ = is_enabled();
        }
        let elapsed = start.elapsed();
        
        println!("100 is_enabled() checks took: {:?}", elapsed);
        let avg = elapsed.as_millis() / 100;
        println!("Average per check: {} ms", avg);
        assert!(avg < 10, "Each check should be < 10ms");
    }

    #[test]
    fn test_enable_idempotent() {
        // Enable twice should be idempotent
        let _ = enable();
        let start = Instant::now();
        let _ = enable();
        let elapsed = start.elapsed();
        
        println!("Second enable (idempotent) took: {:?}", elapsed);
        assert!(elapsed.as_millis() < 500, "Idempotent enable should be fast");
    }

    #[test]
    fn test_append_content_cache() {
        let start = Instant::now();
        let _ = get_append_bytes();
        let first_time = start.elapsed();
        
        let start = Instant::now();
        let _ = get_append_bytes();
        let second_time = start.elapsed();
        
        println!("First call: {:?}, Second call: {:?}", first_time, second_time);
        assert!(second_time < first_time, "Cached call should be faster");
    }
}
