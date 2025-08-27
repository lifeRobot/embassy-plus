use embassy_executor::SpawnToken;
use embassy_hal_internal::Peri;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use log::LevelFilter;
use crate::builder::usb::Irqs;

/// usb log builder
pub struct UsbLogBuilder<'d> {
    /// usb driver
    pub driver: Peri<'d, USB>,
    /// log level
    pub level: LevelFilter,
}

/// custom method
impl UsbLogBuilder<'static> {
    /// create builder
    #[inline]
    pub fn new(level: LevelFilter, usb: Peri<'static, USB>) -> Self {
        Self { level, driver: usb }
    }

    /// create default builder
    #[inline]
    pub fn new_default(usb: Peri<'static, USB>) -> Self {
        Self::new(LevelFilter::Info, usb)
    }

    /// builder logger task
    #[inline]
    pub fn builder(self) -> SpawnToken<impl Sized> {
        logger_task(self)
    }

    /// builder into driver
    #[inline]
    pub fn into_driver(self) -> Driver<'static, USB> {
        Driver::new(self.driver, Irqs)
    }
}

#[embassy_executor::task]
async fn logger_task(builder: UsbLogBuilder<'static>) {
    let device = Driver::new(builder.driver, Irqs);
    embassy_usb_logger::run!(1024, builder.level, device);
}
