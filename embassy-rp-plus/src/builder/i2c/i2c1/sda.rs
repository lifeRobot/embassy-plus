use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_10, PIN_14, PIN_18, PIN_2, PIN_22, PIN_26, PIN_6};

/// i2c1 sda pin
pub enum I2c1Sda<'d> {
    Pin2(Peri<'d, PIN_2>),
    Pin6(Peri<'d, PIN_6>),
    Pin10(Peri<'d, PIN_10>),
    Pin14(Peri<'d, PIN_14>),
    Pin18(Peri<'d, PIN_18>),
    Pin22(Peri<'d, PIN_22>),
    Pin26(Peri<'d, PIN_26>),
}

crate::impl_from!(I2c1Sda::Pin2(PIN_2),"pin_2");
crate::impl_from!(I2c1Sda::Pin6(PIN_6),"pin_6");
crate::impl_from!(I2c1Sda::Pin10(PIN_10),"pin_10");
crate::impl_from!(I2c1Sda::Pin14(PIN_14),"pin_14");
crate::impl_from!(I2c1Sda::Pin18(PIN_18),"pin_18");
crate::impl_from!(I2c1Sda::Pin22(PIN_22),"pin_22");
crate::impl_from!(I2c1Sda::Pin26(PIN_26),"pin_26");
