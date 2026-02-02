//! Unit tests for the Shutdown signal broadcaster.

use super::Shutdown;
use std::time::Duration;
use tokio::time::{error::Elapsed, timeout};

/// Verifies that a new Shutdown has zero subscribers.
#[test]
fn new_shutdown_has_no_subscribers() {
    let shutdown = Shutdown::new();
    assert_eq!(shutdown.subscriber_count(), 0);
}

/// Verifies that Default trait works correctly.
#[test]
fn default_creates_shutdown() {
    let shutdown = Shutdown::default();
    assert_eq!(shutdown.subscriber_count(), 0);
}

/// Verifies that subscribe increments subscriber count.
#[test]
fn subscribe_increments_count() {
    let shutdown = Shutdown::new();
    let _rx1 = shutdown.subscribe();
    assert_eq!(shutdown.subscriber_count(), 1);

    let _rx2 = shutdown.subscribe();
    assert_eq!(shutdown.subscriber_count(), 2);
}

/// Verifies that dropping a receiver decrements subscriber count.
#[test]
fn drop_receiver_decrements_count() {
    let shutdown = Shutdown::new();
    let rx1 = shutdown.subscribe();
    let rx2 = shutdown.subscribe();
    assert_eq!(shutdown.subscriber_count(), 2);

    drop(rx1);
    assert_eq!(shutdown.subscriber_count(), 1);

    drop(rx2);
    assert_eq!(shutdown.subscriber_count(), 0);
}

/// Verifies that cloning Shutdown shares the same channel.
#[test]
fn clone_shares_channel() {
    let shutdown1 = Shutdown::new();
    let shutdown2 = shutdown1.clone();

    let _rx = shutdown1.subscribe();
    assert_eq!(shutdown1.subscriber_count(), 1);
    assert_eq!(shutdown2.subscriber_count(), 1);
}

/// Verifies that trigger notifies subscribers.
#[tokio::test]
async fn trigger_notifies_subscriber() {
    let shutdown = Shutdown::new();
    let mut rx = shutdown.subscribe();

    shutdown.trigger();

    // Should complete immediately since trigger was called
    let result: Result<(), Elapsed> = timeout(Duration::from_millis(100), rx.recv()).await;
    assert!(result.is_ok(), "recv should complete after trigger");
}

/// Verifies that trigger notifies multiple subscribers.
#[tokio::test]
async fn trigger_notifies_multiple_subscribers() {
    let shutdown = Shutdown::new();
    let mut rx1 = shutdown.subscribe();
    let mut rx2 = shutdown.subscribe();

    shutdown.trigger();

    let result1: Result<(), Elapsed> = timeout(Duration::from_millis(100), rx1.recv()).await;
    let result2: Result<(), Elapsed> = timeout(Duration::from_millis(100), rx2.recv()).await;

    assert!(result1.is_ok(), "rx1 should receive signal");
    assert!(result2.is_ok(), "rx2 should receive signal");
}

/// Verifies that trigger is idempotent (multiple calls don't cause issues).
#[tokio::test]
async fn trigger_is_idempotent() {
    let shutdown = Shutdown::new();
    let mut rx = shutdown.subscribe();

    shutdown.trigger();
    shutdown.trigger();
    shutdown.trigger();

    let result: Result<(), Elapsed> = timeout(Duration::from_millis(100), rx.recv()).await;
    assert!(
        result.is_ok(),
        "recv should complete after multiple triggers"
    );
}

/// Verifies that trigger with no subscribers doesn't panic.
#[test]
fn trigger_without_subscribers_succeeds() {
    let shutdown = Shutdown::new();
    shutdown.trigger(); // Should not panic
}

/// Verifies that recv blocks until trigger is called.
#[tokio::test]
async fn recv_blocks_until_trigger() {
    let shutdown = Shutdown::new();
    let mut rx = shutdown.subscribe();

    // recv should not complete immediately
    let result: Result<(), Elapsed> = timeout(Duration::from_millis(50), rx.recv()).await;
    assert!(result.is_err(), "recv should timeout without trigger");

    // Now trigger
    shutdown.trigger();

    // recv should complete
    let result: Result<(), Elapsed> = timeout(Duration::from_millis(100), rx.recv()).await;
    assert!(result.is_ok(), "recv should complete after trigger");
}

/// Verifies that cloned Shutdown can trigger the original's subscribers.
#[tokio::test]
async fn cloned_shutdown_can_trigger() {
    let shutdown1 = Shutdown::new();
    let shutdown2 = shutdown1.clone();

    let mut rx = shutdown1.subscribe();

    shutdown2.trigger();

    let result: Result<(), Elapsed> = timeout(Duration::from_millis(100), rx.recv()).await;
    assert!(
        result.is_ok(),
        "trigger from clone should notify original's subscriber"
    );
}
