use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_11, PIN_27};

/// pwm5 b pin
pub enum Pwm5B<'d> {
    Pin11(Peri<'d, PIN_11>),
    Pin27(Peri<'d, PIN_27>),
}

// support pin.into()
crate::impl_from!(Pwm5B::Pin11(PIN_11),"pin_11");
crate::impl_from!(Pwm5B::Pin27(PIN_27),"pin_27");
