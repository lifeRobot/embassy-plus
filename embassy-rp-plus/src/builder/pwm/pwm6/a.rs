use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_12, PIN_28};

/// pwm6 a pin
pub enum Pwm6A<'d> {
    Pin12(Peri<'d, PIN_12>),
    Pin28(Peri<'d, PIN_28>),
}

// support pin.into()
crate::impl_from!(Pwm6A::Pin12(PIN_12),"pin_12");
crate::impl_from!(Pwm6A::Pin28(PIN_28),"pin_28");