use embassy_hal_internal::Peri;
#[cfg(feature = "pwm0")]
use embassy_rp::peripherals::PWM_SLICE0;
#[cfg(feature = "pwm1")]
use embassy_rp::peripherals::PWM_SLICE1;
#[cfg(feature = "pwm2")]
use embassy_rp::peripherals::PWM_SLICE2;
#[cfg(feature = "pwm3")]
use embassy_rp::peripherals::PWM_SLICE3;
#[cfg(feature = "pwm4")]
use embassy_rp::peripherals::PWM_SLICE4;
#[cfg(feature = "pwm5")]
use embassy_rp::peripherals::PWM_SLICE5;
#[cfg(feature = "pwm6")]
use embassy_rp::peripherals::PWM_SLICE6;
#[cfg(feature = "pwm7")]
use embassy_rp::peripherals::PWM_SLICE7;
#[cfg(feature = "pwm0")]
use crate::builder::pwm::pwm0::Pwm0Builder;
#[cfg(feature = "pwm1")]
use crate::builder::pwm::pwm1::Pwm1Builder;
#[cfg(feature = "pwm2")]
use crate::builder::pwm::pwm2::Pwm2Builder;
#[cfg(feature = "pwm3")]
use crate::builder::pwm::pwm3::Pwm3Builder;
#[cfg(feature = "pwm4")]
use crate::builder::pwm::pwm4::Pwm4Builder;
#[cfg(feature = "pwm5")]
use crate::builder::pwm::pwm5::Pwm5Builder;
#[cfg(feature = "pwm6")]
use crate::builder::pwm::pwm6::Pwm6Builder;
#[cfg(feature = "pwm7")]
use crate::builder::pwm::pwm7::Pwm7Builder;

/// pwm trait
pub trait PwmTrait<B> {
    /// create pwm builder
    fn builder(self) -> B;
}

macro_rules! impl_pwm_trait {
    ($builder:ty,$slice:ty) => {
        /// support pwm trait
        impl<'d> PwmTrait<$builder> for Peri<'d, $slice> {
            #[inline]
            fn builder(self) -> $builder {
                <$builder>::new(self)
            }
        }
    };
}

// support pwm trait
#[cfg(feature = "pwm0")]
impl_pwm_trait!(Pwm0Builder<'d>,PWM_SLICE0);
#[cfg(feature = "pwm1")]
impl_pwm_trait!(Pwm1Builder<'d>,PWM_SLICE1);
#[cfg(feature = "pwm2")]
impl_pwm_trait!(Pwm2Builder<'d>,PWM_SLICE2);
#[cfg(feature = "pwm3")]
impl_pwm_trait!(Pwm3Builder<'d>,PWM_SLICE3);
#[cfg(feature = "pwm4")]
impl_pwm_trait!(Pwm4Builder<'d>,PWM_SLICE4);
#[cfg(feature = "pwm5")]
impl_pwm_trait!(Pwm5Builder<'d>,PWM_SLICE5);
#[cfg(feature = "pwm6")]
impl_pwm_trait!(Pwm6Builder<'d>,PWM_SLICE6);
#[cfg(feature = "pwm7")]
impl_pwm_trait!(Pwm7Builder<'d>,PWM_SLICE7);
