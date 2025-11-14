use embassy_hal_internal::Peri;
use embassy_rp::peripherals::I2C1;
use crate::builder::i2c::i2c1::I2c1Builder;
use crate::builder::i2c::i2c1::scl::I2c1Scl;
use crate::builder::i2c::i2c1::sda::I2c1Sda;

/// i2c1 trait
pub trait I2c1Trait<'d> {
    /// create i2c1 builder
    fn builder(self, scl: impl Into<I2c1Scl<'d>>, sda: impl Into<I2c1Sda<'d>>) -> I2c1Builder<'d>;
}

/// i2c1 support i2c1 trait
impl<'d> I2c1Trait<'d> for Peri<'d, I2C1> {
    #[inline]
    fn builder(self, scl: impl Into<I2c1Scl<'d>>, sda: impl Into<I2c1Sda<'d>>) -> I2c1Builder<'d> {
        I2c1Builder::new(self, scl.into(), sda.into())
    }
}
