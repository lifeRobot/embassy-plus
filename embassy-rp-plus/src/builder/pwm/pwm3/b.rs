use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_23, PIN_7};

/// pwm3 b pin
pub enum Pwm3B<'d> {
    Pin7(Peri<'d, PIN_7>),
    Pin23(Peri<'d, PIN_23>),
}

// support pin.into()
crate::impl_from!(Pwm3B::Pin7(PIN_7),"pin_7");
crate::impl_from!(Pwm3B::Pin23(PIN_23),"pin_23");
