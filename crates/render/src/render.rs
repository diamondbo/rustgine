use core::RustgineSystem;

pub struct RustgineRender;

impl RustgineSystem for RustgineRender {
    fn startup(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
