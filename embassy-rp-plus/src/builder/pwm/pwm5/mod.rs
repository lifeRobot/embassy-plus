use embassy_hal_internal::Peri;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::PWM_SLICE5;
use embassy_rp::pwm::{ChannelAPin, Config, InputMode, Pwm};
use crate::builder::pwm::base::PwmBase;
use crate::builder::pwm::pwm5::a::Pwm5A;
use crate::builder::pwm::pwm5::b::Pwm5B;

pub mod a;
pub mod b;

/// pwm5 builder
pub struct Pwm5Builder<'d> {
    /// pwm5 base data
    pub base: PwmBase<'d, PWM_SLICE5>,
}

/// custom method
impl<'d> Pwm5Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(pwm: Peri<'d, PWM_SLICE5>) -> Self {
        Self { base: PwmBase::new(pwm) }
    }

    /// set pwm config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// Create PWM driver without any configured pins.<br />
    /// more see [Pwm::new_free]
    #[inline]
    pub fn free(self) -> Pwm<'d> {
        Pwm::new_free(self.base.pwm, self.base.config.unwrap_or_default())
    }

    /// Create PWM driver with a single 'a' pin as output.<br />
    /// more see [Pwm::new_output_a]
    #[inline]
    pub fn output_a(self, a: impl Into<Pwm5A<'d>>) -> Pwm<'d> {
        match a.into() {
            Pwm5A::Pin10(pin_10) => Pwm::new_output_a(self.base.pwm, pin_10, self.base.config.unwrap_or_default()),
            Pwm5A::Pin26(pin_26) => Pwm::new_output_a(self.base.pwm, pin_26, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' pin as output.<br />
    /// more see [Pwm::new_output_b]
    #[inline]
    pub fn output_b(self, b: impl Into<Pwm5B<'d>>) -> Pwm<'d> {
        match b.into() {
            Pwm5B::Pin11(pin_11) => Pwm::new_output_b(self.base.pwm, pin_11, self.base.config.unwrap_or_default()),
            Pwm5B::Pin27(pin_27) => Pwm::new_output_b(self.base.pwm, pin_27, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins as output.<br />
    /// more see [Pwm::new_output_ab]
    #[inline]
    pub fn output_ab(self, a: impl Into<Pwm5A<'d>>, b: impl Into<Pwm5B<'d>>) -> Pwm<'d> {
        match a.into() {
            Pwm5A::Pin10(pin_10) => self.build_b(pin_10, b),
            Pwm5A::Pin26(pin_26) => self.build_b(pin_26, b),
        }
    }

    /// build output_ab by b
    #[inline]
    fn build_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE5>>, b: impl Into<Pwm5B<'d>>) -> Pwm<'d> {
        match b.into() {
            Pwm5B::Pin11(pin_11) => Pwm::new_output_ab(self.base.pwm, a, pin_11, self.base.config.unwrap_or_default()),
            Pwm5B::Pin27(pin_27) => Pwm::new_output_ab(self.base.pwm, a, pin_27, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' as input pin.<br />
    /// more see [Pwm::new_input]
    #[inline]
    pub fn input(self, b: impl Into<Pwm5B<'d>>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b.into() {
            Pwm5B::Pin11(pin_11) => Pwm::new_input(self.base.pwm, pin_11, pull, mode, self.base.config.unwrap_or_default()),
            Pwm5B::Pin27(pin_27) => Pwm::new_input(self.base.pwm, pin_27, pull, mode, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins in the desired input mode.<br />
    /// more see [Pwm::new_output_input]
    #[inline]
    pub fn output_input(self, a: impl Into<Pwm5A<'d>>, b: impl Into<Pwm5B<'d>>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match a.into() {
            Pwm5A::Pin10(pin_10) => self.input_b(pin_10, b, pull, mode),
            Pwm5A::Pin26(pin_26) => self.input_b(pin_26, b, pull, mode),
        }
    }

    /// build output_input by b
    #[inline]
    fn input_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE5>>, b: impl Into<Pwm5B<'d>>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b.into() {
            Pwm5B::Pin11(pin_11) => Pwm::new_output_input(self.base.pwm, a, pin_11, pull, mode, self.base.config.unwrap_or_default()),
            Pwm5B::Pin27(pin_27) => Pwm::new_output_input(self.base.pwm, a, pin_27, pull, mode, self.base.config.unwrap_or_default()),
        }
    }
}
