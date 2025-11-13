use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_1, PIN_13, PIN_17, PIN_21, PIN_25, PIN_29, PIN_5, PIN_9};

/// i2c0 scl pin
pub enum I2c0Scl<'d> {
    Pin1(Peri<'d, PIN_1>),
    Pin5(Peri<'d, PIN_5>),
    Pin9(Peri<'d, PIN_9>),
    Pin13(Peri<'d, PIN_13>),
    Pin17(Peri<'d, PIN_17>),
    Pin21(Peri<'d, PIN_21>),
    Pin25(Peri<'d, PIN_25>),
    Pin29(Peri<'d, PIN_29>),
}

crate::impl_from!(I2c0Scl::Pin1(PIN_1),"pin_1");
crate::impl_from!(I2c0Scl::Pin5(PIN_5),"pin_5");
crate::impl_from!(I2c0Scl::Pin9(PIN_9),"pin_9");
crate::impl_from!(I2c0Scl::Pin13(PIN_13),"pin_13");
crate::impl_from!(I2c0Scl::Pin17(PIN_17),"pin_17");
crate::impl_from!(I2c0Scl::Pin21(PIN_21),"pin_21");
crate::impl_from!(I2c0Scl::Pin25(PIN_25),"pin_25");
crate::impl_from!(I2c0Scl::Pin29(PIN_29),"pin_29");
