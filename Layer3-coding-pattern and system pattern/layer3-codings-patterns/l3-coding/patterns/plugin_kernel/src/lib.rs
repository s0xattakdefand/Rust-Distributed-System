//! Micro-kernel / Plugin – load `.so` or `.dll` at runtime.

use anyhow::Result;
use libloading::{Library, Symbol};

/// Host-side trait signature.
pub trait Plugin { fn name(&self) -> &'static str; }

/// Dynamically load a plugin and call `name()`.
pub fn load_plugin(path: &str) -> Result<String> {
    unsafe {
        let lib = Library::new(path)?;
        // plugin exposes `extern "C" fn create() -> Box<dyn Plugin>`
        let ctor: Symbol<unsafe extern "C" fn() -> Box<dyn Plugin>> = lib.get(b"create")?;
        Ok(ctor().name().into())
    }
}

#[cfg(test)]
mod tests {
    // Just ensure it links; real test would dlopen a compiled plugin.
    #[test] fn api_exists() {
        use super::*; fn _dummy(_: &str) -> Result<String> { Ok("x".into()) }
    }
}
