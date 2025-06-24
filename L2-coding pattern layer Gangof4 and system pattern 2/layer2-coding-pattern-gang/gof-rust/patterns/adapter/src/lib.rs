// TODO: src
//! Convert Lightning iPhone plug → USB-C laptop port.

pub trait UsbC {
    fn send_power(&self) -> &'static str;
}

/// Adaptee we cannot change
pub struct LightningPlug;
impl LightningPlug { pub fn charge(&self) -> &'static str { "power via Lightning" } }

/// Object adapter
pub struct LightningToUsbCAdapter { plug: LightningPlug }
impl LightningToUsbCAdapter {
    pub fn new(plug: LightningPlug) -> Self { Self { plug } }
}
impl UsbC for LightningToUsbCAdapter {
    fn send_power(&self) -> &'static str { self.plug.charge() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn adapt() {
        let laptop_port: &dyn UsbC = &LightningToUsbCAdapter::new(LightningPlug);
        assert!(laptop_port.send_power().contains("Lightning"));
    }
}
