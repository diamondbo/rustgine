use core::RustgineSystem;

pub struct RustgineScheduler;

impl RustgineSystem for RustgineScheduler {
    fn startup(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
