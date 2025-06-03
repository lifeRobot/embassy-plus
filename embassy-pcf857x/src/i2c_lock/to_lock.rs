use embassy_rp::i2c::{I2c, Instance, Mode};
use crate::i2c_lock::I2cLock;

/// to i2c lock
pub trait ToLock<'a, T: Instance, M: Mode> {
    /// to i2c lock
    fn to_lock(self) -> I2cLock<'a, T, M>;
}

/// i2c support to lock
impl<'a, T: Instance, M: Mode> ToLock<'a, T, M> for I2c<'a, T, M> {
    #[inline]
    fn to_lock(self) -> I2cLock<'a, T, M> {
        I2cLock::new(self)
    }
}
