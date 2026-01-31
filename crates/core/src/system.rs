/// Trait for engine systems with startup/shutdown hooks.
pub trait RustgineSystem {
    /// Called once at engine startup for initialization.
    fn startup(&mut self) -> anyhow::Result<()>;

    /// Called once at engine shutdown for cleanup.
    fn shutdown(&mut self) -> anyhow::Result<()>;
}
