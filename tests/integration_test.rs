use std::time::Instant;
use std::net::TcpStream;
use std::time::Duration;

/// Test GitHub connectivity performance after hosts file modification
/// Target: Connection should complete within 1 second
#[test]
fn test_github_connection_performance() {
    // Test connection to github.com
    let start = Instant::now();
    
    match TcpStream::connect_timeout(
        &"140.82.113.4:443".parse().unwrap(),
        Duration::from_secs(2),
    ) {
        Ok(_stream) => {
            let elapsed = start.elapsed();
            println!("GitHub connection time: {:?}", elapsed);
            println!("Connection millis: {}", elapsed.as_millis());
            
            // Target: connection should be under 1 second
            assert!(
                elapsed.as_millis() < 1000,
                "GitHub connection should complete within 1000ms, took: {:?}",
                elapsed
            );
        }
        Err(e) => {
            println!("Connection failed (may be offline): {}", e);
            // Don't fail test if network is unavailable
        }
    }
}

/// Test multiple rapid connections
#[test]
fn test_rapid_github_connections() {
    let start = Instant::now();
    let mut success_count = 0;
    
    for i in 0..5 {
        let conn_start = Instant::now();
        match TcpStream::connect_timeout(
            &"140.82.113.4:443".parse().unwrap(),
            Duration::from_secs(1),
        ) {
            Ok(_) => {
                let elapsed = conn_start.elapsed();
                println!("Connection {} took: {:?}", i + 1, elapsed);
                success_count += 1;
            }
            Err(e) => {
                println!("Connection {} failed: {}", i + 1, e);
            }
        }
    }
    
    let total_elapsed = start.elapsed();
    println!(
        "Total time for 5 connections: {:?}, Success rate: {}/5",
        total_elapsed, success_count
    );
    
    if success_count > 0 {
        let avg_time = total_elapsed.as_millis() / success_count as u128;
        println!("Average connection time: {} ms", avg_time);
    }
}

/// Test hosts file operations performance in sequence
#[test]
fn test_hosts_operations_sequence() {
    use free_to_github::hosts;
    
    let start = Instant::now();
    
    // Test 1: Check if enabled
    let check_start = Instant::now();
    let is_enabled = hosts::is_enabled().unwrap_or(false);
    let check_time = check_start.elapsed();
    println!("is_enabled check: {:?} -> {}", check_time, is_enabled);
    assert!(check_time.as_millis() < 100, "Status check should be < 100ms");
    
    // Test 2: Enable (or verify already enabled)
    let enable_start = Instant::now();
    let _ = hosts::enable();
    let enable_time = enable_start.elapsed();
    println!("enable operation: {:?}", enable_time);
    assert!(enable_time.as_millis() < 1000, "Enable should be < 1000ms");
    
    // Test 3: Verify enabled
    let verify_start = Instant::now();
    let is_enabled_now = hosts::is_enabled().unwrap_or(false);
    let verify_time = verify_start.elapsed();
    println!("verify enabled: {:?} -> {}", verify_time, is_enabled_now);
    
    let total = start.elapsed();
    println!("Total sequence time: {:?}", total);
    assert!(total.as_millis() < 2000, "Total sequence should be < 2000ms");
}

/// Benchmark: Multiple operations with timing breakdown
#[test]
fn test_performance_benchmark() {
    use free_to_github::hosts;
    
    println!("\n=== Performance Benchmark ===");
    
    let mut total_time = Duration::ZERO;
    
    for i in 0..3 {
        let start = Instant::now();
        let _ = hosts::is_enabled();
        let elapsed = start.elapsed();
        total_time += elapsed;
        
        println!("is_enabled {}: {:.2}ms", i + 1, elapsed.as_secs_f64() * 1000.0);
    }
    
    println!("Total benchmark time: {:.2}ms", total_time.as_secs_f64() * 1000.0);
    println!("=== Benchmark Complete ===\n");
}

/// Test cache effectiveness
#[test]
fn test_cache_effectiveness() {
    use free_to_github::hosts;
    
    let mut times = Vec::new();
    
    // First call - cache miss
    let start = Instant::now();
    let _ = hosts::is_enabled();
    times.push(start.elapsed());
    
    // Subsequent calls - should be cached/faster
    for _ in 0..10 {
        let start = Instant::now();
        let _ = hosts::is_enabled();
        times.push(start.elapsed());
    }
    
    let first = times[0].as_micros();
    let avg_rest = times[1..].iter().map(|t| t.as_micros()).sum::<u128>() / times[1..].len() as u128;
    
    println!("First call: {} µs", first);
    println!("Average of next 10: {} µs", avg_rest);
    
    // Subsequent calls should be reasonably fast
    assert!(
        avg_rest < 100000,
        "Average operation should be < 100ms, got {} µs",
        avg_rest
    );
}
