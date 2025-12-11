use embassy_hal_internal::Peri;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::PWM_SLICE3;
use embassy_rp::pwm::{ChannelAPin, Config, InputMode, Pwm};
use crate::builder::pwm::base::PwmBase;
use crate::builder::pwm::pwm3::a::Pwm3A;
use crate::builder::pwm::pwm3::b::Pwm3B;

pub mod a;
pub mod b;

/// pwm3 builder
pub struct Pwm3Builder<'d> {
    /// pwm3 base data
    pub base: PwmBase<'d, PWM_SLICE3>,
}

/// custom method
impl<'d> Pwm3Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(pwm: Peri<'d, PWM_SLICE3>) -> Self {
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
    pub fn output_a(self, a: Pwm3A<'d>) -> Pwm<'d> {
        match a {
            Pwm3A::Pin6(pin_6) => Pwm::new_output_a(self.base.pwm, pin_6, self.base.config.unwrap_or_default()),
            Pwm3A::Pin22(pin_22) => Pwm::new_output_a(self.base.pwm, pin_22, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' pin as output.<br />
    /// more see [Pwm::new_output_b]
    #[inline]
    pub fn output_b(self, b: Pwm3B<'d>) -> Pwm<'d> {
        match b {
            Pwm3B::Pin7(pin_7) => Pwm::new_output_b(self.base.pwm, pin_7, self.base.config.unwrap_or_default()),
            Pwm3B::Pin23(pin_23) => Pwm::new_output_b(self.base.pwm, pin_23, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins as output.<br />
    /// more see [Pwm::new_output_ab]
    #[inline]
    pub fn output_ab(self, a: Pwm3A<'d>, b: Pwm3B<'d>) -> Pwm<'d> {
        match a {
            Pwm3A::Pin6(pin_6) => self.build_b(pin_6, b),
            Pwm3A::Pin22(pin_22) => self.build_b(pin_22, b),
        }
    }

    /// build output_ab by b
    #[inline]
    fn build_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE3>>, b: Pwm3B<'d>) -> Pwm<'d> {
        match b {
            Pwm3B::Pin7(pin_7) => Pwm::new_output_ab(self.base.pwm, a, pin_7, self.base.config.unwrap_or_default()),
            Pwm3B::Pin23(pin_23) => Pwm::new_output_ab(self.base.pwm, a, pin_23, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a single 'b' as input pin.<br />
    /// more see [Pwm::new_input]
    #[inline]
    pub fn input(self, b: Pwm3B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b {
            Pwm3B::Pin7(pin_7) => Pwm::new_input(self.base.pwm, pin_7, pull, mode, self.base.config.unwrap_or_default()),
            Pwm3B::Pin23(pin_23) => Pwm::new_input(self.base.pwm, pin_23, pull, mode, self.base.config.unwrap_or_default()),
        }
    }

    /// Create PWM driver with a 'a' and 'b' pins in the desired input mode.<br />
    /// more see [Pwm::new_output_input]
    #[inline]
    pub fn output_input(self, a: Pwm3A<'d>, b: Pwm3B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match a {
            Pwm3A::Pin6(pin_6) => self.input_b(pin_6, b, pull, mode),
            Pwm3A::Pin22(pin_22) => self.input_b(pin_22, b, pull, mode),
        }
    }

    /// build output_input by b
    #[inline]
    fn input_b(self, a: Peri<'d, impl ChannelAPin<PWM_SLICE3>>, b: Pwm3B<'d>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        match b {
            Pwm3B::Pin7(pin_7) => Pwm::new_output_input(self.base.pwm, a, pin_7, pull, mode, self.base.config.unwrap_or_default()),
            Pwm3B::Pin23(pin_23) => Pwm::new_output_input(self.base.pwm, a, pin_23, pull, mode, self.base.config.unwrap_or_default()),
        }
    }
}
