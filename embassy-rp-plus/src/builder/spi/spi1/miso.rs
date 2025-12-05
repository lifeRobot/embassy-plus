use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_12, PIN_24, PIN_28, PIN_8};

/// spi1 miso pin
pub enum Spi1Miso<'d> {
    Pin8(Peri<'d, PIN_8>),
    Pin12(Peri<'d, PIN_12>),
    Pin24(Peri<'d, PIN_24>),
    Pin28(Peri<'d, PIN_28>),
}

// support pin.into()
crate::impl_from!(Spi1Miso::Pin8(PIN_8),"pin_8");
crate::impl_from!(Spi1Miso::Pin12(PIN_12),"pin_12");
crate::impl_from!(Spi1Miso::Pin24(PIN_24),"pin_24");
crate::impl_from!(Spi1Miso::Pin28(PIN_28),"pin_28");
