use std::time::Duration;
use wagg::dispatcher::{
    calculate_retry_delay, ATTEMPT_TIMEOUT, BASE_DELAY, IDLE_RECHECK, MAX_ATTEMPTS, MAX_DB_DELAY,
    MAX_DELAY,
};

/// [WACDP-7B] Exponential Backoff Retry Engine
/// Validates mathematical sequence progression, caps, timeouts,
/// and retry boundaries.

#[test]
fn test_exponential_backoff_sequence_progression() {
    // Attempt 1: BASE_DELAY (500ms)
    let delay_1 = calculate_retry_delay(1);
    assert_eq!(delay_1, Duration::from_millis(500));

    // Attempt 2: 1,000ms (2x)
    let delay_2 = calculate_retry_delay(2);
    assert_eq!(delay_2, Duration::from_millis(1000));

    // Attempt 3: 2,000ms (4x)
    let delay_3 = calculate_retry_delay(3);
    assert_eq!(delay_3, Duration::from_millis(2000));

    // Attempt 4: 4,000ms (8x)
    let delay_4 = calculate_retry_delay(4);
    assert_eq!(delay_4, Duration::from_millis(4000));

    // Attempt 5: 8,000ms (16x)
    let delay_5 = calculate_retry_delay(5);
    assert_eq!(delay_5, Duration::from_millis(8000));
}

#[test]
fn test_exponential_backoff_cap_at_max_delay() {
    // Attempt 6: 16s
    let delay_6 = calculate_retry_delay(6);
    assert_eq!(delay_6, Duration::from_secs(16));

    // Attempt 7: 32s would exceed 30s cap, so must cap at MAX_DELAY (30s)
    let delay_7 = calculate_retry_delay(7);
    assert_eq!(delay_7, MAX_DELAY);
    assert_eq!(delay_7, Duration::from_secs(30));

    // Attempt 10: also capped at 30s
    let delay_10 = calculate_retry_delay(10);
    assert_eq!(delay_10, MAX_DELAY);
}

#[test]
fn test_system_timing_constants() {
    assert_eq!(MAX_ATTEMPTS, 5, "MAX_ATTEMPTS must be 5");
    assert_eq!(ATTEMPT_TIMEOUT, Duration::from_secs(10), "ATTEMPT_TIMEOUT must be 10s");
    assert_eq!(BASE_DELAY, Duration::from_millis(500), "BASE_DELAY must be 500ms");
    assert_eq!(MAX_DELAY, Duration::from_secs(30), "MAX_DELAY must be 30s");
    assert_eq!(MAX_DB_DELAY, Duration::from_secs(5), "MAX_DB_DELAY must be 5s");
    assert_eq!(IDLE_RECHECK, Duration::from_secs(5), "IDLE_RECHECK must be 5s");
}

#[test]
fn test_db_retry_delay_exponential_capping() {
    let mut db_delay = BASE_DELAY;
    let mut delays = Vec::new();

    for _ in 0..6 {
        delays.push(db_delay);
        db_delay = (db_delay * 2).min(MAX_DB_DELAY);
    }

    assert_eq!(delays[0], Duration::from_millis(500));
    assert_eq!(delays[1], Duration::from_millis(1000));
    assert_eq!(delays[2], Duration::from_millis(2000));
    assert_eq!(delays[3], Duration::from_millis(4000));
    assert_eq!(delays[4], Duration::from_millis(5000)); // capped at 5s
    assert_eq!(delays[5], Duration::from_millis(5000)); // stays at 5s
}

