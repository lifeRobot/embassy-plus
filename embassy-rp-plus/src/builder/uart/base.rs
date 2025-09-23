use embassy_hal_internal::{Peri, PeripheralType};
use embassy_rp::uart::Config;

/// uart base data
pub struct UartBase<'d, T: PeripheralType> {
    /// uart devices
    pub uart: Peri<'d, T>,
    /// uart config
    pub config: Option<Config>,
}

/// custom method
impl<'d, T: PeripheralType> UartBase<'d, T> {
    /// create base data
    #[inline]
    pub fn new(uart: Peri<'d, T>) -> Self {
        Self { uart, config: None }
    }

    /// set uart config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// set uart config
    #[inline]
    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config)
    }
}
