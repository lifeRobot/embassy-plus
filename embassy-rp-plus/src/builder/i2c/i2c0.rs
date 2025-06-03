use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{Async, Config, I2c, InterruptHandler};
use embassy_rp::peripherals::{I2C0, PIN_20, PIN_21};

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

/// i2c1 builder
pub struct I2c0Builder {
    /// ic21 device
    pub i2c0: I2C0,
    /// scl pin
    pub scl: PIN_21,
    /// sda pin
    pub sda: PIN_20,
    /// i2c config
    pub config: Config,
}

/// custom method
impl I2c0Builder {
    /// create i2c1 builder
    #[inline]
    pub fn new(i2c0: I2C0, scl: PIN_21, sda: PIN_20) -> Self {
        Self { i2c0, scl, sda, config: Config::default() }
    }

    /// build i2c device
    #[inline]
    pub fn build(self) -> I2c<'static, I2C0, Async> {
        I2c::new_async(self.i2c0, self.scl, self.sda, Irqs, self.config)
    }
}
