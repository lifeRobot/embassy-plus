use embassy_hal_internal::Peri;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{Async, Blocking, Config, I2c, InterruptHandler, SclPin};
use embassy_rp::peripherals::I2C1;
use crate::builder::i2c::i2c1::scl::I2c1Scl;
use crate::builder::i2c::i2c1::sda::I2c1Sda;

pub mod scl;
pub mod sda;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});

/// i2c1 builder
pub struct I2c1Builder<'d> {
    /// ic21 device
    pub i2c1: Peri<'d, I2C1>,
    /// scl pin
    pub scl: I2c1Scl<'d>,
    /// sda pin
    pub sda: I2c1Sda<'d>,
    /// i2c config
    pub config: Config,
}

/// custom method
impl<'d> I2c1Builder<'d> {
    /// create i2c1 builder
    #[inline]
    pub fn new(i2c1: Peri<'d, I2C1>, scl: I2c1Scl<'d>, sda: I2c1Sda<'d>) -> Self {
        Self { i2c1, scl, sda, config: Config::default() }
    }

    /// build i2c device
    #[inline]
    pub fn build(self) -> I2c<'d, I2C1, Async> {
        match self.scl {
            I2c1Scl::Pin3(pin_2) => Self::build_sda(self.i2c1, pin_2, self.sda, self.config),
            I2c1Scl::Pin7(pin_7) => Self::build_sda(self.i2c1, pin_7, self.sda, self.config),
            I2c1Scl::Pin11(pin_11) => Self::build_sda(self.i2c1, pin_11, self.sda, self.config),
            I2c1Scl::Pin15(pin_15) => Self::build_sda(self.i2c1, pin_15, self.sda, self.config),
            I2c1Scl::Pin19(pin_19) => Self::build_sda(self.i2c1, pin_19, self.sda, self.config),
            I2c1Scl::Pin23(pin_23) => Self::build_sda(self.i2c1, pin_23, self.sda, self.config),
            I2c1Scl::Pin27(pin_27) => Self::build_sda(self.i2c1, pin_27, self.sda, self.config),
        }
    }

    /// build i2c by sda
    fn build_sda(i2c: Peri<'d, I2C1>, scl: Peri<'d, impl SclPin<I2C1>>, sda: I2c1Sda<'d>, config: Config) -> I2c<'d, I2C1, Async> {
        match sda {
            I2c1Sda::Pin2(pin_2) => I2c::new_async(i2c, scl, pin_2, Irqs, config),
            I2c1Sda::Pin6(pin_6) => I2c::new_async(i2c, scl, pin_6, Irqs, config),
            I2c1Sda::Pin10(pin_10) => I2c::new_async(i2c, scl, pin_10, Irqs, config),
            I2c1Sda::Pin14(pin_14) => I2c::new_async(i2c, scl, pin_14, Irqs, config),
            I2c1Sda::Pin18(pin_18) => I2c::new_async(i2c, scl, pin_18, Irqs, config),
            I2c1Sda::Pin22(pin_22) => I2c::new_async(i2c, scl, pin_22, Irqs, config),
            I2c1Sda::Pin26(pin_26) => I2c::new_async(i2c, scl, pin_26, Irqs, config),
        }
    }

    /// build i2c device
    #[inline]
    pub fn build_blocking(self) -> I2c<'d, I2C1, Blocking> {
        match self.scl {
            I2c1Scl::Pin3(pin_2) => Self::build_blocking_sda(self.i2c1, pin_2, self.sda, self.config),
            I2c1Scl::Pin7(pin_7) => Self::build_blocking_sda(self.i2c1, pin_7, self.sda, self.config),
            I2c1Scl::Pin11(pin_11) => Self::build_blocking_sda(self.i2c1, pin_11, self.sda, self.config),
            I2c1Scl::Pin15(pin_15) => Self::build_blocking_sda(self.i2c1, pin_15, self.sda, self.config),
            I2c1Scl::Pin19(pin_19) => Self::build_blocking_sda(self.i2c1, pin_19, self.sda, self.config),
            I2c1Scl::Pin23(pin_23) => Self::build_blocking_sda(self.i2c1, pin_23, self.sda, self.config),
            I2c1Scl::Pin27(pin_27) => Self::build_blocking_sda(self.i2c1, pin_27, self.sda, self.config),
        }
    }

    /// build i2c by sda
    fn build_blocking_sda(i2c: Peri<'d, I2C1>, scl: Peri<'d, impl SclPin<I2C1>>, sda: I2c1Sda<'d>, config: Config) -> I2c<'d, I2C1, Blocking> {
        match sda {
            I2c1Sda::Pin2(pin_2) => I2c::new_blocking(i2c, scl, pin_2, config),
            I2c1Sda::Pin6(pin_6) => I2c::new_blocking(i2c, scl, pin_6, config),
            I2c1Sda::Pin10(pin_10) => I2c::new_blocking(i2c, scl, pin_10, config),
            I2c1Sda::Pin14(pin_14) => I2c::new_blocking(i2c, scl, pin_14, config),
            I2c1Sda::Pin18(pin_18) => I2c::new_blocking(i2c, scl, pin_18, config),
            I2c1Sda::Pin22(pin_22) => I2c::new_blocking(i2c, scl, pin_22, config),
            I2c1Sda::Pin26(pin_26) => I2c::new_blocking(i2c, scl, pin_26, config),
        }
    }
}
