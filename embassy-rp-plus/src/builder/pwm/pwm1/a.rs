use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_18, PIN_2};

/// pwm1 a pin
pub enum Pwm1A<'d> {
    Pin2(Peri<'d, PIN_2>),
    Pin18(Peri<'d, PIN_18>),
}

// support pin.into()
crate::impl_from!(Pwm1A::Pin2(PIN_2),"pin_2");
crate::impl_from!(Pwm1A::Pin18(PIN_18),"pin_18");
