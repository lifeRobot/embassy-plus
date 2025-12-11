use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_0, PIN_16};

/// pwm0 a pin
pub enum Pwm0A<'d> {
    Pin0(Peri<'d, PIN_0>),
    Pin16(Peri<'d, PIN_16>),
}

// support pin.into()
crate::impl_from!(Pwm0A::Pin0(PIN_0),"pin_0");
crate::impl_from!(Pwm0A::Pin16(PIN_16),"pin_16");
