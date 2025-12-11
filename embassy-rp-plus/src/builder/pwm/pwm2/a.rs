use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_20, PIN_4};

/// pwm2 a pin
pub enum Pwm2A<'d> {
    Pin4(Peri<'d, PIN_4>),
    Pin20(Peri<'d, PIN_20>),
}

// support pin.into()
crate::impl_from!(Pwm2A::Pin4(PIN_4),"pin_4");
crate::impl_from!(Pwm2A::Pin20(PIN_20),"pin_20");
