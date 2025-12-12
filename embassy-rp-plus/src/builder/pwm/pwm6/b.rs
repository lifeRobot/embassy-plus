use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_13, PIN_29};

/// pwm6 b pin
pub enum Pwm6B<'d> {
    Pin13(Peri<'d, PIN_13>),
    Pin29(Peri<'d, PIN_29>),
}

// support pin.into()
crate::impl_from!(Pwm6B::Pin13(PIN_13),"pin_13");
crate::impl_from!(Pwm6B::Pin29(PIN_29),"pin_29");