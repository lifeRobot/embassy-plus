use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_0, PIN_16, PIN_20, PIN_4};

/// spi0 miso pin
pub enum Spi0Miso<'d> {
    Pin0(Peri<'d, PIN_0>),
    Pin4(Peri<'d, PIN_4>),
    Pin16(Peri<'d, PIN_16>),
    Pin20(Peri<'d, PIN_20>),
}

// support pin.into()
crate::impl_from!(Spi0Miso::Pin0(PIN_0),"pin_0");
crate::impl_from!(Spi0Miso::Pin4(PIN_4),"pin_4");
crate::impl_from!(Spi0Miso::Pin16(PIN_16),"pin_16");
crate::impl_from!(Spi0Miso::Pin20(PIN_20),"pin_20");