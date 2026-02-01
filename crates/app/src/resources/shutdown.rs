use std::sync::Arc;
use tokio::sync::broadcast;

/// Graceful shutdown signal broadcaster for the application.
///
/// Allows multiple tasks to subscribe to a shutdown signal and react when triggered.
#[derive(Clone)]
pub struct Shutdown {
    inner: Arc<broadcast::Sender<()>>,
}

impl Default for Shutdown {
    fn default() -> Self {
        Self::new()
    }
}

impl Shutdown {
    /// Create a new Shutdown broadcaster.
    ///
    /// # Example
    /// ```
    /// use app::resources::Shutdown;
    /// let shutdown = Shutdown::new();
    /// ```
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(8);
        Shutdown {
            inner: Arc::new(tx),
        }
    }

    /// Trigger the shutdown signal, notifying all subscribers.
    ///
    /// This can be called multiple times, but only the first call will notify all receivers.
    pub fn trigger(&self) {
        // Ignore send errors: it just means no one is listening.
        let _ = self.inner.send(());
    }

    /// Subscribe to the shutdown signal.
    ///
    /// Returns a ShutdownRx which can be awaited for the shutdown event.
    pub fn subscribe(&self) -> ShutdownRx {
        ShutdownRx {
            rx: self.inner.subscribe(),
        }
    }
}

/// Receiver for the shutdown signal.
///
/// Awaiting `recv()` will complete when shutdown is triggered.
pub struct ShutdownRx {
    rx: broadcast::Receiver<()>,
}

impl ShutdownRx {
    /// Wait for the shutdown signal.
    ///
    /// Returns when the shutdown is triggered.
    pub async fn recv(&mut self) {
        // Ignore errors: RecvError::Closed or RecvError::Lagged are both treated as shutdown.
        let _ = self.rx.recv().await;
    }
}
