use embassy_hal_internal::Peri;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{Async, Config, I2c, InterruptHandler};
use embassy_rp::peripherals::{I2C0, PIN_20, PIN_21};

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

/// i2c1 builder
pub struct I2c0Builder<'d> {
    /// ic21 device
    pub i2c0: Peri<'d, I2C0>,
    /// scl pin
    pub scl: Peri<'d, PIN_21>,
    /// sda pin
    pub sda: Peri<'d, PIN_20>,
    /// i2c config
    pub config: Config,
}

/// custom method
impl<'d> I2c0Builder<'d> {
    /// create i2c1 builder
    #[inline]
    pub fn new(i2c0: Peri<'d, I2C0>, scl: Peri<'d, PIN_21>, sda: Peri<'d, PIN_20>) -> Self {
        Self { i2c0, scl, sda, config: Config::default() }
    }

    /// build i2c device
    #[inline]
    pub fn build(self) -> I2c<'d, I2C0, Async> {
        I2c::new_async(self.i2c0, self.scl, self.sda, Irqs, self.config)
    }
}
