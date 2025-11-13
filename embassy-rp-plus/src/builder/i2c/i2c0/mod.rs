use embassy_hal_internal::Peri;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{Async, Blocking, Config, I2c, InterruptHandler, SclPin};
use embassy_rp::peripherals::I2C0;
use crate::builder::i2c::i2c0::scl::I2c0Scl;
use crate::builder::i2c::i2c0::sda::I2c0Sda;

pub mod scl;
pub mod sda;

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

/// i2c1 builder
pub struct I2c0Builder<'d> {
    /// ic21 device
    pub i2c0: Peri<'d, I2C0>,
    /// scl pin
    pub scl: I2c0Scl<'d>,
    /// sda pin
    pub sda: I2c0Sda<'d>,
    /// i2c config
    pub config: Config,
}

/// custom method
impl<'d> I2c0Builder<'d> {
    /// create i2c1 builder
    #[inline]
    pub fn new(i2c0: Peri<'d, I2C0>, scl: I2c0Scl<'d>, sda: I2c0Sda<'d>) -> Self {
        Self { i2c0, scl, sda, config: Config::default() }
    }

    /// build i2c device
    #[inline]
    pub fn build(self) -> I2c<'d, I2C0, Async> {
        match self.scl {
            I2c0Scl::Pin1(pin_1) => Self::build_sda(self.i2c0, pin_1, self.sda, self.config),
            I2c0Scl::Pin5(pin_5) => Self::build_sda(self.i2c0, pin_5, self.sda, self.config),
            I2c0Scl::Pin9(pin_9) => Self::build_sda(self.i2c0, pin_9, self.sda, self.config),
            I2c0Scl::Pin13(pin_13) => Self::build_sda(self.i2c0, pin_13, self.sda, self.config),
            I2c0Scl::Pin17(pin_17) => Self::build_sda(self.i2c0, pin_17, self.sda, self.config),
            I2c0Scl::Pin21(pin_21) => Self::build_sda(self.i2c0, pin_21, self.sda, self.config),
            I2c0Scl::Pin25(pin_25) => Self::build_sda(self.i2c0, pin_25, self.sda, self.config),
            I2c0Scl::Pin29(pin_29) => Self::build_sda(self.i2c0, pin_29, self.sda, self.config),
        }
    }

    /// build i2c by sda
    fn build_sda(i2c: Peri<'d, I2C0>, scl: Peri<'d, impl SclPin<I2C0>>, sda: I2c0Sda<'d>, config: Config) -> I2c<'d, I2C0, Async> {
        match sda {
            I2c0Sda::Pin0(pin_0) => I2c::new_async(i2c, scl, pin_0, Irqs, config),
            I2c0Sda::Pin4(pin_4) => I2c::new_async(i2c, scl, pin_4, Irqs, config),
            I2c0Sda::Pin8(pin_8) => I2c::new_async(i2c, scl, pin_8, Irqs, config),
            I2c0Sda::Pin12(pin_12) => I2c::new_async(i2c, scl, pin_12, Irqs, config),
            I2c0Sda::Pin16(pin_16) => I2c::new_async(i2c, scl, pin_16, Irqs, config),
            I2c0Sda::Pin20(pin_20) => I2c::new_async(i2c, scl, pin_20, Irqs, config),
            I2c0Sda::Pin24(pin_24) => I2c::new_async(i2c, scl, pin_24, Irqs, config),
            I2c0Sda::Pin28(pin_28) => I2c::new_async(i2c, scl, pin_28, Irqs, config),
        }
    }

    /// build i2c device
    #[inline]
    pub fn build_blocking(self) -> I2c<'d, I2C0, Blocking> {
        match self.scl {
            I2c0Scl::Pin1(pin_1) => Self::build_blocking_sda(self.i2c0, pin_1, self.sda, self.config),
            I2c0Scl::Pin5(pin_5) => Self::build_blocking_sda(self.i2c0, pin_5, self.sda, self.config),
            I2c0Scl::Pin9(pin_9) => Self::build_blocking_sda(self.i2c0, pin_9, self.sda, self.config),
            I2c0Scl::Pin13(pin_13) => Self::build_blocking_sda(self.i2c0, pin_13, self.sda, self.config),
            I2c0Scl::Pin17(pin_17) => Self::build_blocking_sda(self.i2c0, pin_17, self.sda, self.config),
            I2c0Scl::Pin21(pin_21) => Self::build_blocking_sda(self.i2c0, pin_21, self.sda, self.config),
            I2c0Scl::Pin25(pin_25) => Self::build_blocking_sda(self.i2c0, pin_25, self.sda, self.config),
            I2c0Scl::Pin29(pin_29) => Self::build_blocking_sda(self.i2c0, pin_29, self.sda, self.config),
        }
    }

    /// build i2c by sda
    fn build_blocking_sda(i2c: Peri<'d, I2C0>, scl: Peri<'d, impl SclPin<I2C0>>, sda: I2c0Sda<'d>, config: Config) -> I2c<'d, I2C0, Blocking> {
        match sda {
            I2c0Sda::Pin0(pin_0) => I2c::new_blocking(i2c, scl, pin_0, config),
            I2c0Sda::Pin4(pin_4) => I2c::new_blocking(i2c, scl, pin_4, config),
            I2c0Sda::Pin8(pin_8) => I2c::new_blocking(i2c, scl, pin_8, config),
            I2c0Sda::Pin12(pin_12) => I2c::new_blocking(i2c, scl, pin_12, config),
            I2c0Sda::Pin16(pin_16) => I2c::new_blocking(i2c, scl, pin_16, config),
            I2c0Sda::Pin20(pin_20) => I2c::new_blocking(i2c, scl, pin_20, config),
            I2c0Sda::Pin24(pin_24) => I2c::new_blocking(i2c, scl, pin_24, config),
            I2c0Sda::Pin28(pin_28) => I2c::new_blocking(i2c, scl, pin_28, config),
        }
    }
}
