use embassy_hal_internal::Peri;
use embassy_rp::peripherals::USB;
use log::LevelFilter;
use crate::builder::usb::usb_log::UsbLogBuilder;

/// usb log trait
pub trait UsbLogTrait: Sized {
    /// create usb log builder
    fn log_builder(self, level: LevelFilter) -> UsbLogBuilder<'static>;

    /// create usb log builder by default
    #[inline]
    fn log_builder_default(self) -> UsbLogBuilder<'static> {
        Self::log_builder(self, LevelFilter::Info)
    }
}

/// support USB
impl UsbLogTrait for Peri<'static, USB> {
    #[inline]
    fn log_builder(self, level: LevelFilter) -> UsbLogBuilder<'static> {
        UsbLogBuilder::new(level, self)
    }
}
