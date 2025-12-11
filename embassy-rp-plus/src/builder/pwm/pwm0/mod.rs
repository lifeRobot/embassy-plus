use embassy_hal_internal::Peri;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::PWM_SLICE0;
use embassy_rp::pwm::{ChannelAPin, Config, InputMode, Pwm};
use crate::builder::pwm::base::PwmBase;
use crate::builder::pwm::pwm0::a::Pwm0A;
use crate::builder::pwm::pwm0::b::Pwm0B;

pub mod a;
pub mod b;

/// pwm0 builder
pub struct Pwm0Builder<'d> {
    /// pwm0 base data
    pub base: PwmBase<'d, PWM_SLICE0>,
}

/// custom method
impl<'d> Pwm0Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(pwm: Peri<'d, PWM_SLICE0>) -> Self {
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
    pub fn output_a(self, a: Pwm0A<'d>) -> Pwm<'d> {
        match a {
            Pwm0A::Pin0(pin_0) => Pwm::new_output_a(self.base.pwm, pin_0, self.base.config.unwrap_or_default()),
            Pwm0A::Pin16(pin_16) => Pwm::new_output_a(self.base.pwm, pin_16, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' pin as output.<br />
    /// more see [Pwm::new_output_b]
    #[inline]
    pub fn output_b(self, b: Pwm0B<'d>) -> Pwm<'d> {
        match b {
            Pwm0B::Pin1(pin_1) => Pwm::new_output_b(self.base.pwm, pin_1, self.base.config.unwrap_or_default()),
            Pwm0B::Pin17(pin_17) => Pwm::new_output_b(self.base.pwm, pin_17, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins as output.<br />
    /// more see [Pwm::new_output_ab]
    #[inline]
    pub fn output_ab(self, a: Pwm0A<'d>, b: Pwm0B<'d>) -> Pwm<'d> {
        match a {
            Pwm0A::Pin0(pin_0) => self.build_b(pin_0, b),
            Pwm0A::Pin16(pin_16) => self.build_b(pin_16, b),
        }
    }

    /// build output_ab by b
    #[inline]
    fn build_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE0>>, b: Pwm0B<'d>) -> Pwm<'d> {
        match b {
            Pwm0B::Pin1(pin_1) => Pwm::new_output_ab(self.base.pwm, a, pin_1, self.base.config.unwrap_or_default()),
            Pwm0B::Pin17(pin_17) => Pwm::new_output_ab(self.base.pwm, a, pin_17, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' as input pin.<br />
    /// more see [Pwm::new_input]
    #[inline]
    pub fn input(self, b: Pwm0B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b {
            Pwm0B::Pin1(pin_1) => Pwm::new_input(self.base.pwm, pin_1, pull, mode, self.base.config.unwrap_or_default()),
            Pwm0B::Pin17(pin_17) => Pwm::new_input(self.base.pwm, pin_17, pull, mode, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins in the desired input mode.<br />
    /// more see [Pwm::new_output_input]
    #[inline]
    pub fn output_input(self, a: Pwm0A<'d>, b: Pwm0B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match a {
            Pwm0A::Pin0(pin_0) => self.input_b(pin_0, b, pull, mode),
            Pwm0A::Pin16(pin_16) => self.input_b(pin_16, b, pull, mode),
        }
    }

    /// build output_input by b
    #[inline]
    fn input_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE0>>, b: Pwm0B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b {
            Pwm0B::Pin1(pin_1) => Pwm::new_output_input(self.base.pwm, a, pin_1, pull, mode, self.base.config.unwrap_or_default()),
            Pwm0B::Pin17(pin_17) => Pwm::new_output_input(self.base.pwm, a, pin_17, pull, mode, self.base.config.unwrap_or_default()),
        }
    }
}
