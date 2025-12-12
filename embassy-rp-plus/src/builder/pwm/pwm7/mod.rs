use embassy_hal_internal::Peri;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::{PIN_14, PIN_15, PWM_SLICE7};
use embassy_rp::pwm::{Config, InputMode, Pwm};
use crate::builder::pwm::base::PwmBase;

/// pwm7 builder
pub struct Pwm7Builder<'d> {
    /// pwm7 base data
    pub base: PwmBase<'d, PWM_SLICE7>,
}

/// custom method
impl<'d> Pwm7Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(pwm: Peri<'d, PWM_SLICE7>) -> Self {
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
    pub fn output_a(self, a: Peri<'d, PIN_14>) -> Pwm<'d> {
        Pwm::new_output_a(self.base.pwm, a, self.base.config.unwrap_or_default())
    }

    /// Create PWM driver with a single 'b' pin as output.<br />
    /// more see [Pwm::new_output_b]
    #[inline]
    pub fn output_b(self, b: Peri<'d, PIN_15>) -> Pwm<'d> {
        Pwm::new_output_b(self.base.pwm, b, self.base.config.unwrap_or_default())
    }

    /// Create PWM driver with a 'a' and 'b' pins as output.<br />
    /// more see [Pwm::new_output_ab]
    #[inline]
    pub fn output_ab(self, a: Peri<'d, PIN_14>, b: Peri<'d, PIN_15>) -> Pwm<'d> {
        Pwm::new_output_ab(self.base.pwm, a, b, self.base.config.unwrap_or_default())
    }

    /// Create PWM driver with a single 'b' as input pin.<br />
    /// more see [Pwm::new_input]
    #[inline]
    pub fn input(self, b: Peri<'d, PIN_15>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        Pwm::new_input(self.base.pwm, b, pull, mode, self.base.config.unwrap_or_default())
    }

    /// Create PWM driver with a 'a' and 'b' pins in the desired input mode.<br />
    /// more see [Pwm::new_output_input]
    #[inline]
    pub fn output_input(self, a: Peri<'d, PIN_14>, b: Peri<'d, PIN_15>, pull: Pull, mode: InputMode) -> Pwm<'d> {
        Pwm::new_output_input(self.base.pwm, a, b, pull, mode, self.base.config.unwrap_or_default())
    }
}
