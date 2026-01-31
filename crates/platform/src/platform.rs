use core::RustgineSystem;

pub struct RustginePlatform;

impl RustgineSystem for RustginePlatform {
    fn startup(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
