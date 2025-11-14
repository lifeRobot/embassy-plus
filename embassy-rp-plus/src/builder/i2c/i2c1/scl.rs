use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_11, PIN_15, PIN_19, PIN_23, PIN_27, PIN_3, PIN_7};

/// i2c1 scl pin
pub enum I2c1Scl<'d> {
    Pin3(Peri<'d, PIN_3>),
    Pin7(Peri<'d, PIN_7>),
    Pin11(Peri<'d, PIN_11>),
    Pin15(Peri<'d, PIN_15>),
    Pin19(Peri<'d, PIN_19>),
    Pin23(Peri<'d, PIN_23>),
    Pin27(Peri<'d, PIN_27>),
}

crate::impl_from!(I2c1Scl::Pin3(PIN_3),"pin_3");
crate::impl_from!(I2c1Scl::Pin7(PIN_7),"pin_7");
crate::impl_from!(I2c1Scl::Pin11(PIN_11),"pin_11");
crate::impl_from!(I2c1Scl::Pin15(PIN_15),"pin_15");
crate::impl_from!(I2c1Scl::Pin19(PIN_19),"pin_19");
crate::impl_from!(I2c1Scl::Pin23(PIN_23),"pin_23");
crate::impl_from!(I2c1Scl::Pin27(PIN_27),"pin_27");
