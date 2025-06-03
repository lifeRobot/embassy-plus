use embassy_rp::Peripheral;
use embassy_rp::peripherals::USB;
use log::LevelFilter;
use crate::builder::usb::usb_log::UsbLogBuilder;

/// usb log trait
pub trait UsbLogTrait: Peripheral<P=USB> {
    /// create usb log builder
    fn log_builder(self, level: LevelFilter) -> UsbLogBuilder;

    /// create usb log builder by default
    #[inline]
    fn log_builder_default(self) -> UsbLogBuilder {
        Self::log_builder(self, LevelFilter::Info)
    }
}

/// support USB
impl UsbLogTrait for USB {
    #[inline]
    fn log_builder(self, level: LevelFilter) -> UsbLogBuilder {
        UsbLogBuilder::new(level, self)
    }
}
