//! Graceful shutdown signal management.
//!
//! Provides a broadcast-based shutdown signaling mechanism that allows
//! multiple tasks to coordinate graceful termination.

use std::sync::Arc;
use tokio::sync::broadcast;

/// Default channel capacity for shutdown signal broadcasting.
///
/// This value should be large enough to handle all subscribers but
/// small enough to avoid excessive memory allocation.
const SHUTDOWN_CHANNEL_CAPACITY: usize = 16;

/// Graceful shutdown signal broadcaster for the application.
///
/// Enables coordinated shutdown across multiple async tasks by broadcasting
/// a shutdown signal that all subscribers receive simultaneously.
///
/// # Thread Safety
///
/// `Shutdown` is [`Clone`] and [`Send`] + [`Sync`], making it safe to share
/// across threads and async tasks.
///
/// # Design Notes
///
/// Uses a broadcast channel internally, which allows:
/// - Multiple subscribers to receive the same signal
/// - Late subscribers to still receive pending signals
/// - Minimal overhead when no subscribers exist
///
/// # Example
///
/// ```
/// use app::resources::Shutdown;
///
/// let shutdown = Shutdown::new();
///
/// // Subscribe from multiple tasks
/// let mut rx1 = shutdown.subscribe();
/// let mut rx2 = shutdown.subscribe();
///
/// // Trigger shutdown (typically on Ctrl+C or error)
/// shutdown.trigger();
/// ```
#[derive(Clone, Debug)]
pub struct Shutdown {
    /// The underlying broadcast sender.
    ///
    /// Wrapped in `Arc` to allow cheap cloning while maintaining
    /// a single broadcast channel instance.
    sender: Arc<broadcast::Sender<()>>,
}

impl Default for Shutdown {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Shutdown {
    /// Creates a new shutdown signal broadcaster.
    ///
    /// # Example
    ///
    /// ```
    /// use app::resources::Shutdown;
    ///
    /// let shutdown = Shutdown::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(SHUTDOWN_CHANNEL_CAPACITY);
        Self {
            sender: Arc::new(sender),
        }
    }

    /// Triggers the shutdown signal, notifying all active subscribers.
    ///
    /// This method is idempotent - calling it multiple times has no
    /// additional effect after the first call.
    ///
    /// # Notes
    ///
    /// - Send errors are silently ignored (indicates no active receivers)
    /// - Subscribers that join after triggering will not receive the signal
    #[inline]
    pub fn trigger(&self) {
        // Ignore send errors: indicates no active receivers, which is fine.
        let _ = self.sender.send(());
    }

    /// Subscribes to the shutdown signal.
    ///
    /// Returns a [`ShutdownRx`] that can be awaited to detect when
    /// shutdown has been triggered.
    ///
    /// # Example
    ///
    /// ```
    /// use app::resources::Shutdown;
    ///
    /// let shutdown = Shutdown::new();
    /// let mut rx = shutdown.subscribe();
    ///
    /// // In an async context:
    /// // rx.recv().await;
    /// ```
    #[must_use]
    #[inline]
    pub fn subscribe(&self) -> ShutdownRx {
        ShutdownRx {
            receiver: self.sender.subscribe(),
        }
    }

    /// Returns the number of active subscribers.
    ///
    /// Useful for debugging and testing shutdown coordination.
    #[must_use]
    #[inline]
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

/// Receiver for shutdown signals.
///
/// Obtained by calling [`Shutdown::subscribe`]. Awaiting [`recv`](Self::recv)
/// will complete when the shutdown signal is triggered.
///
/// # Cancellation Safety
///
/// The `recv` method is cancellation-safe. If the future is dropped before
/// completion, no signal is lost.
#[derive(Debug)]
pub struct ShutdownRx {
    receiver: broadcast::Receiver<()>,
}

impl ShutdownRx {
    /// Waits for the shutdown signal.
    ///
    /// Returns immediately if shutdown has already been triggered.
    /// Otherwise, blocks until the signal is received.
    ///
    /// # Notes
    ///
    /// - Returns on any receiver error (closed channel, lagged receiver)
    /// - Safe to call multiple times (subsequent calls return immediately)
    #[inline]
    pub async fn recv(&mut self) {
        // Treat all errors as shutdown signal:
        // - RecvError::Closed: sender dropped, shutdown implied
        // - RecvError::Lagged: missed messages, treat as shutdown
        let _ = self.receiver.recv().await;
    }
}
