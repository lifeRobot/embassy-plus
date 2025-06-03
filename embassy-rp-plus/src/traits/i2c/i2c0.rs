use embassy_rp::peripherals::{I2C0, PIN_20, PIN_21};
use crate::builder::i2c::i2c0::I2c0Builder;

/// i2c0 trait
pub trait I2c0Trait {
    /// create i2c0 builder
    fn builder(self, scl: PIN_21, sda: PIN_20) -> I2c0Builder;
}

/// i2c0 support i2c0 trait
impl I2c0Trait for I2C0 {
    #[inline]
    fn builder(self, scl: PIN_21, sda: PIN_20) -> I2c0Builder {
        I2c0Builder::new(self, scl, sda)
    }
}
