use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_19, PIN_23, PIN_3, PIN_7};

/// spi0 mosi pin
pub enum Spi0Mosi<'d> {
    Pin3(Peri<'d, PIN_3>),
    Pin7(Peri<'d, PIN_7>),
    Pin19(Peri<'d, PIN_19>),
    Pin23(Peri<'d, PIN_23>),
}

// support pin.into()
crate::impl_from!(Spi0Mosi::Pin3(PIN_3),"pin_3");
crate::impl_from!(Spi0Mosi::Pin7(PIN_7),"pin_7");
crate::impl_from!(Spi0Mosi::Pin19(PIN_19),"pin_19");
crate::impl_from!(Spi0Mosi::Pin23(PIN_23),"pin_23");
