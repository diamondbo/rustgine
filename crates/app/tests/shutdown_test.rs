use app::resources::Shutdown;

#[tokio::test]
async fn shutdown_triggers_receivers() {
    let shutdown = Shutdown::new();
    let mut rx1 = shutdown.subscribe();
    let mut rx2 = shutdown.subscribe();
    shutdown.trigger();
    rx1.recv().await;
    rx2.recv().await;
}
