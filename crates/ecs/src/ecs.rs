use core::RustgineSystem;

pub struct RustgineEcs;

impl RustgineSystem for RustgineEcs {
    fn startup(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
