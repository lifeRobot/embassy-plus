use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{I2C0, PIN_20, PIN_21};
use crate::builder::i2c::i2c0::I2c0Builder;

/// i2c0 trait
pub trait I2c0Trait<'d> {
    /// create i2c0 builder
    fn builder(self, scl: Peri<'d, PIN_21>, sda: Peri<'d, PIN_20>) -> I2c0Builder<'d>;
}

/// i2c0 support i2c0 trait
impl<'d> I2c0Trait<'d> for Peri<'d, I2C0> {
    #[inline]
    fn builder(self, scl: Peri<'d, PIN_21>, sda: Peri<'d, PIN_20>) -> I2c0Builder<'d> {
        I2c0Builder::new(self, scl, sda)
    }
}
