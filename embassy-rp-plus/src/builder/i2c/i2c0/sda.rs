use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_0, PIN_12, PIN_16, PIN_20, PIN_24, PIN_28, PIN_4, PIN_8};

/// i2c0 sda pin
pub enum I2c0Sda<'d> {
    Pin0(Peri<'d, PIN_0>),
    Pin4(Peri<'d, PIN_4>),
    Pin8(Peri<'d, PIN_8>),
    Pin12(Peri<'d, PIN_12>),
    Pin16(Peri<'d, PIN_16>),
    Pin20(Peri<'d, PIN_20>),
    Pin24(Peri<'d, PIN_24>),
    Pin28(Peri<'d, PIN_28>),
}

crate::impl_from!(I2c0Sda::Pin0(PIN_0),"pin_0");
crate::impl_from!(I2c0Sda::Pin4(PIN_4),"pin_4");
crate::impl_from!(I2c0Sda::Pin8(PIN_8),"pin_8");
crate::impl_from!(I2c0Sda::Pin12(PIN_12),"pin_12");
crate::impl_from!(I2c0Sda::Pin16(PIN_16),"pin_16");
crate::impl_from!(I2c0Sda::Pin20(PIN_20),"pin_20");
crate::impl_from!(I2c0Sda::Pin24(PIN_24),"pin_24");
crate::impl_from!(I2c0Sda::Pin28(PIN_28),"pin_28");
