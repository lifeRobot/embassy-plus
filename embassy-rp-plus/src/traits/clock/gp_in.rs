use embassy_hal_internal::Peri;
use embassy_rp::clocks::{Gpin, GpinPin};

/// general purpose clock inputs trait
pub trait GpInTrait<'d, T: GpinPin> {
    /// create Gpin driver, more see [Gpin::new]
    fn in_put(self) -> Gpin<'d, T>;
}

/// any support GpinPin support GpInTrait
impl<'d, T: GpinPin> GpInTrait<'d, T> for Peri<'d, T> {
    #[inline]
    fn in_put(self) -> Gpin<'d, T> {
        Gpin::new(self)
    }
}
