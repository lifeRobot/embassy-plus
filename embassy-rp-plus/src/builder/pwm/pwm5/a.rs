use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_10, PIN_26};

/// pwm5 a pin
pub enum Pwm5A<'d> {
    Pin10(Peri<'d, PIN_10>),
    Pin26(Peri<'d, PIN_26>),
}

// support pin.into()
crate::impl_from!(Pwm5A::Pin10(PIN_10),"pin_10");
crate::impl_from!(Pwm5A::Pin26(PIN_26),"pin_26");
