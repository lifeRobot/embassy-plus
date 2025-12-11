use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_24, PIN_8};

/// pwm4 a pin
pub enum Pwm4A<'d> {
    Pin8(Peri<'d, PIN_8>),
    Pin24(Peri<'d, PIN_24>),
}

// support pin.into()
crate::impl_from!(Pwm4A::Pin8(PIN_8),"pin_8");
crate::impl_from!(Pwm4A::Pin24(PIN_24),"pin_24");
