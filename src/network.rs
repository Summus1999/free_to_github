//! Network module for IP latency testing and smart IP selection
//!
//! This module provides functionality to:
//! - Test TCP connection latency to GitHub IPs
//! - Select the fastest IP for each domain
//! - Support multiple candidate IPs per domain

use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// Connection timeout for latency test (milliseconds)
const CONNECT_TIMEOUT_MS: u64 = 3000;

/// Port to use for testing (HTTPS)
const TEST_PORT: u16 = 443;

/// Result of a single IP latency test
#[derive(Debug, Clone)]
pub struct LatencyResult {
    pub ip: String,
    pub domain: String,
    pub latency_ms: Option<u64>,
    pub success: bool,
}

/// Domain with multiple candidate IPs
#[derive(Debug, Clone)]
pub struct DomainEntry {
    pub domain: String,
    pub candidate_ips: Vec<String>,
    pub best_ip: Option<String>,
    pub best_latency_ms: Option<u64>,
}

/// GitHub domains with multiple candidate IPs for smart selection
pub fn get_domain_candidates() -> Vec<DomainEntry> {
    vec![
        DomainEntry {
            domain: "github.com".to_string(),
            candidate_ips: vec![
                "140.82.112.4".to_string(),
                "140.82.113.4".to_string(),
                "140.82.114.4".to_string(),
                "20.205.243.166".to_string(),
                "20.27.177.113".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "api.github.com".to_string(),
            candidate_ips: vec![
                "140.82.112.6".to_string(),
                "140.82.113.6".to_string(),
                "140.82.114.6".to_string(),
                "20.205.243.168".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "gist.github.com".to_string(),
            candidate_ips: vec![
                "140.82.112.4".to_string(),
                "140.82.113.4".to_string(),
                "140.82.114.4".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "raw.githubusercontent.com".to_string(),
            candidate_ips: vec![
                "185.199.108.133".to_string(),
                "185.199.109.133".to_string(),
                "185.199.110.133".to_string(),
                "185.199.111.133".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "codeload.github.com".to_string(),
            candidate_ips: vec![
                "140.82.112.10".to_string(),
                "140.82.113.10".to_string(),
                "140.82.114.10".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "github.githubassets.com".to_string(),
            candidate_ips: vec![
                "185.199.108.154".to_string(),
                "185.199.109.154".to_string(),
                "185.199.110.154".to_string(),
                "185.199.111.154".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "assets-cdn.github.com".to_string(),
            candidate_ips: vec![
                "185.199.108.153".to_string(),
                "185.199.109.153".to_string(),
                "185.199.110.153".to_string(),
                "185.199.111.153".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "cloud.githubusercontent.com".to_string(),
            candidate_ips: vec![
                "185.199.108.133".to_string(),
                "185.199.109.133".to_string(),
                "185.199.110.133".to_string(),
                "185.199.111.133".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "avatars.githubusercontent.com".to_string(),
            candidate_ips: vec![
                "185.199.108.133".to_string(),
                "185.199.109.133".to_string(),
                "185.199.110.133".to_string(),
                "185.199.111.133".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "github.global.ssl.fastly.net".to_string(),
            candidate_ips: vec![
                "199.232.69.194".to_string(),
                "151.101.1.194".to_string(),
                "151.101.65.194".to_string(),
                "151.101.129.194".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "githubstatus.com".to_string(),
            candidate_ips: vec![
                "185.199.108.153".to_string(),
                "185.199.109.153".to_string(),
                "185.199.110.153".to_string(),
                "185.199.111.153".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "collector.github.com".to_string(),
            candidate_ips: vec![
                "140.82.112.22".to_string(),
                "140.82.113.22".to_string(),
                "140.82.114.22".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
        DomainEntry {
            domain: "objects.githubusercontent.com".to_string(),
            candidate_ips: vec![
                "185.199.108.133".to_string(),
                "185.199.109.133".to_string(),
                "185.199.110.133".to_string(),
                "185.199.111.133".to_string(),
            ],
            best_ip: None,
            best_latency_ms: None,
        },
    ]
}

/// Test TCP connection latency to a single IP
pub fn test_ip_latency(ip: &str) -> Option<u64> {
    let addr = format!("{}:{}", ip, TEST_PORT);
    let socket_addr = match addr.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr,
            None => return None,
        },
        Err(_) => return None,
    };

    let timeout = Duration::from_millis(CONNECT_TIMEOUT_MS);
    let start = Instant::now();

    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_stream) => {
            let elapsed = start.elapsed().as_millis() as u64;
            Some(elapsed)
        }
        Err(_) => None,
    }
}

/// Test all candidate IPs for a domain and find the fastest one
pub fn find_best_ip_for_domain(entry: &mut DomainEntry) -> Option<LatencyResult> {
    let mut best_result: Option<LatencyResult> = None;

    for ip in &entry.candidate_ips {
        if let Some(latency) = test_ip_latency(ip) {
            let result = LatencyResult {
                ip: ip.clone(),
                domain: entry.domain.clone(),
                latency_ms: Some(latency),
                success: true,
            };

            match &best_result {
                None => best_result = Some(result),
                Some(current_best) => {
                    if let Some(current_latency) = current_best.latency_ms {
                        if latency < current_latency {
                            best_result = Some(result);
                        }
                    }
                }
            }
        }
    }

    // Update entry with best result
    if let Some(ref result) = best_result {
        entry.best_ip = Some(result.ip.clone());
        entry.best_latency_ms = result.latency_ms;
    }

    best_result
}

/// Speed test progress callback type
pub type ProgressCallback = Box<dyn Fn(usize, usize, &str) + Send + Sync>;

/// Test all domains in parallel and find the best IP for each
/// Returns a map of domain -> (best_ip, latency_ms)
pub fn test_all_domains_parallel(
    progress_callback: Option<Arc<dyn Fn(usize, usize, &str) + Send + Sync>>,
) -> HashMap<String, (String, u64)> {
    let domains = get_domain_candidates();
    let total = domains.len();
    let results: Arc<Mutex<HashMap<String, (String, u64)>>> = Arc::new(Mutex::new(HashMap::new()));
    let completed = Arc::new(Mutex::new(0usize));

    let mut handles = vec![];

    for mut entry in domains {
        let results = Arc::clone(&results);
        let completed = Arc::clone(&completed);
        let progress_callback = progress_callback.clone();
        let domain = entry.domain.clone();

        let handle = thread::spawn(move || {
            if let Some(best) = find_best_ip_for_domain(&mut entry) {
                if let Some(latency) = best.latency_ms {
                    let mut res = results.lock().unwrap();
                    res.insert(best.domain.clone(), (best.ip, latency));
                }
            }

            // Update progress
            let mut count = completed.lock().unwrap();
            *count += 1;
            let current = *count;
            drop(count);

            if let Some(ref cb) = progress_callback {
                cb(current, total, &domain);
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join();
    }

    let final_results = results.lock().unwrap().clone();
    final_results
}

/// Quick test of key domains only (github.com, api, raw)
pub fn test_key_domains() -> Vec<LatencyResult> {
    let key_domains = vec!["github.com", "api.github.com", "raw.githubusercontent.com"];
    let mut results = vec![];

    let all_domains = get_domain_candidates();
    for mut entry in all_domains {
        if key_domains.contains(&entry.domain.as_str()) {
            if let Some(result) = find_best_ip_for_domain(&mut entry) {
                results.push(result);
            } else {
                results.push(LatencyResult {
                    ip: entry.candidate_ips.first().cloned().unwrap_or_default(),
                    domain: entry.domain,
                    latency_ms: None,
                    success: false,
                });
            }
        }
    }

    results
}

/// Get latency quality rating
pub fn get_quality_rating(latency_ms: u64) -> &'static str {
    match latency_ms {
        0..=50 => "极佳",
        51..=100 => "良好",
        101..=200 => "一般",
        201..=500 => "较慢",
        _ => "很慢",
    }
}

/// Get latency quality color (RGB)
pub fn get_quality_color(latency_ms: u64) -> (u8, u8, u8) {
    match latency_ms {
        0..=50 => (80, 220, 120),      // Green - Excellent
        51..=100 => (120, 200, 80),    // Light green - Good
        101..=200 => (220, 180, 50),   // Yellow - Fair
        201..=500 => (255, 140, 60),   // Orange - Slow
        _ => (255, 80, 80),            // Red - Very slow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_ip_latency() {
        // Test a known GitHub IP
        let result = test_ip_latency("140.82.113.4");
        println!("Latency to 140.82.113.4: {:?} ms", result);
        // We don't assert success since network might be unavailable
    }

    #[test]
    fn test_domain_candidates() {
        let domains = get_domain_candidates();
        assert!(!domains.is_empty());
        assert!(domains.iter().any(|d| d.domain == "github.com"));
    }

    #[test]
    fn test_quality_rating() {
        assert_eq!(get_quality_rating(30), "极佳");
        assert_eq!(get_quality_rating(80), "良好");
        assert_eq!(get_quality_rating(150), "一般");
        assert_eq!(get_quality_rating(300), "较慢");
        assert_eq!(get_quality_rating(1000), "很慢");
    }
}
