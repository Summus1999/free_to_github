use std::time::Instant;
use std::net::TcpStream;
use std::time::Duration;
use free_to_github::logger;

// Initialize logger for tests
fn init_logger() {
    let _ = logger::FileLogger::init();
}

/// Test GitHub connectivity performance after hosts file modification
/// Target: Connection should complete within 1 second
#[test]
fn test_github_connection_performance() {
    init_logger();
    
    #[cfg(debug_assertions)]
    log::info!("=== Starting GitHub connection diagnostic test ===");
    
    // Test connection to github.com
    let start = Instant::now();
    
    #[cfg(debug_assertions)]
    log::info!("Attempting to connect to github.com:443 (140.82.113.4:443)");
    
    let dns_start = Instant::now();
    let addr = match "140.82.113.4:443".parse() {
        Ok(a) => {
            #[cfg(debug_assertions)]
            log::info!("Address parsing took {} µs", dns_start.elapsed().as_micros());
            a
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            log::error!("Failed to parse address: {}", e);
            return;
        }
    };
    
    let connect_start = Instant::now();
    match TcpStream::connect_timeout(&addr, Duration::from_secs(2)) {
        Ok(_stream) => {
            let total_elapsed = start.elapsed();
            let connect_elapsed = connect_start.elapsed();
            let duration_ms = total_elapsed.as_millis();
            
            #[cfg(debug_assertions)]
            {
                log::info!("✓ TCP connection established");
                log::info!("Connection handshake time: {} ms", connect_elapsed.as_millis());
                log::info!("Total connection time: {} ms", duration_ms);
                log::info!("Connection status: SUCCESS");
            }
            
            println!("GitHub connection time: {:?}", total_elapsed);
            println!("Connection millis: {}", duration_ms);
            
            logger::log_connection_metrics("github.com:443", duration_ms, true);
            
            // Target: connection should be under 1 second
            assert!(
                duration_ms < 1000,
                "GitHub connection should complete within 1000ms, took: {:?}",
                total_elapsed
            );
        }
        Err(e) => {
            let elapsed = start.elapsed();
            let elapsed_ms = elapsed.as_millis();
            
            #[cfg(debug_assertions)]
            {
                log::warn!("✗ TCP connection failed");
                log::warn!("Error type: {}", e);
                log::warn!("Time elapsed before timeout: {} ms", elapsed_ms);
                log::warn!("Error details: {:?}", e.kind());
                log::warn!("Possible causes:");
                log::warn!("  - Network unreachable");
                log::warn!("  - Firewall blocking connection");
                log::warn!("  - GitHub server not responding");
                log::warn!("  - DNS resolution failed");
            }
            
            println!("Connection failed (may be offline): {}", e);
            logger::log_connection_metrics("github.com:443", elapsed_ms, false);
            // Don't fail test if network is unavailable
        }
    }
    
    #[cfg(debug_assertions)]
    log::info!("=== GitHub connection diagnostic test complete ===");
}

/// Test multiple rapid connections
#[test]
fn test_rapid_github_connections() {
    init_logger();
    
    #[cfg(debug_assertions)]
    log::info!("=== Starting rapid connection test (5 attempts) ===");
    
    let start = Instant::now();
    let mut success_count = 0;
    let mut total_time = 0u128;
    
    for i in 0..5 {
        #[cfg(debug_assertions)]
        log::info!("Attempt {}: Starting connection...", i + 1);
        
        let conn_start = Instant::now();
        match TcpStream::connect_timeout(
            &"140.82.113.4:443".parse().unwrap(),
            Duration::from_secs(1),
        ) {
            Ok(_) => {
                let elapsed = conn_start.elapsed();
                let duration_ms = elapsed.as_millis();
                total_time += duration_ms;
                
                #[cfg(debug_assertions)]
                log::info!("Attempt {}: ✓ SUCCESS - {} ms", i + 1, duration_ms);
                
                println!("Connection {} took: {:?}", i + 1, elapsed);
                logger::log_connection_metrics(
                    &format!("github.com:443 (attempt {})", i + 1),
                    duration_ms,
                    true
                );
                success_count += 1;
            }
            Err(e) => {
                let duration_ms = conn_start.elapsed().as_millis();
                total_time += duration_ms;
                
                #[cfg(debug_assertions)]
                log::warn!("Attempt {}: ✗ FAILED - {} ms - Error: {}", i + 1, duration_ms, e);
                
                println!("Connection {} failed: {}", i + 1, e);
                logger::log_connection_metrics(
                    &format!("github.com:443 (attempt {})", i + 1),
                    duration_ms,
                    false
                );
            }
        }
    }
    
    let total_elapsed = start.elapsed();
    
    #[cfg(debug_assertions)]
    {
        log::info!("=== Rapid connection test results ===");
        log::info!("Total attempts: 5");
        log::info!("Successful: {}", success_count);
        log::info!("Failed: {}", 5 - success_count);
        log::info!("Success rate: {}/5", success_count);
        log::info!("Total elapsed time: {} ms", total_elapsed.as_millis());
        if success_count > 0 {
            let avg_time = total_time / success_count as u128;
            log::info!("Average connection time: {} ms", avg_time);
            if avg_time > 1000 {
                log::warn!("WARNING: Average connection time exceeds 1 second (performance issue)");
            }
        }
    }
    
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
    init_logger();
    
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
    init_logger();
    
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
