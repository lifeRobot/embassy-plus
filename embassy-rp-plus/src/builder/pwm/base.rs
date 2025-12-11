use embassy_hal_internal::Peri;
use embassy_rp::pwm::{Config, Slice};

/// pwm base data
pub struct PwmBase<'d, T: Slice> {
    /// pwm devices
    pub pwm: Peri<'d, T>,
    /// pwm config
    pub config: Option<Config>,
}

/// custom method
impl<'d, T: Slice> PwmBase<'d, T> {
    /// create base data
    #[inline]
    pub fn new(pwm: Peri<'d, T>) -> Self {
        Self { pwm, config: None }
    }

    /// set pwm config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// set pwm config
    #[inline]
    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config)
    }
}
