use embassy_hal_internal::Peri;
use embassy_rp::clocks::{Gpout, GpoutPin};

/// general purpose clock outputs trait
pub trait GpOutTrait<'d, T: GpoutPin> {
    /// create Gpout driver, more see [Gpout::new]
    fn gp_out(self) -> Gpout<'d, T>;
}

/// any support GpoutPin support GpOutTrait
impl<'d, T: GpoutPin> GpOutTrait<'d, T> for Peri<'d, T> {
    #[inline]
    fn gp_out(self) -> Gpout<'d, T> {
        Gpout::new(self)
    }
}
