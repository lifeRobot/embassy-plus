use embassy_hal_internal::{Peri, PeripheralType};
use embassy_rp::spi::Config;

/// spi base data
pub struct SpiBase<'d, T: PeripheralType> {
    /// spi devices
    pub spi: Peri<'d, T>,
    /// spi config
    pub config: Option<Config>,
}

/// custom method
impl<'d, T: PeripheralType> SpiBase<'d, T> {
    /// create base data
    #[inline]
    pub fn new(spi: Peri<'d, T>) -> Self {
        Self { spi, config: None }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// set spi config
    #[inline]
    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config)
    }
}
