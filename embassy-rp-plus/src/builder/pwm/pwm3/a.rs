use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_22, PIN_6};

/// pwm3 a pin
pub enum Pwm3A<'d> {
    Pin6(Peri<'d, PIN_6>),
    Pin22(Peri<'d, PIN_22>),
}

// support pin.into()
crate::impl_from!(Pwm3A::Pin6(PIN_6),"pin_6");
crate::impl_from!(Pwm3A::Pin22(PIN_22),"pin_22");
