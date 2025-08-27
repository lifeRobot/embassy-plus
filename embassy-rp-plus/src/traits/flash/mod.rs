use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::flash::{Async, Blocking, Flash};
use embassy_rp::peripherals::FLASH;

/// flash trait
pub trait FlashTrait<'d> {
    /// create flash driver<br />
    /// more see [Flash::new]
    fn build<const FLASH_SIZE: usize>(self, dma: Peri<'d, impl Channel>) -> Flash<'d, FLASH, Async, FLASH_SIZE>;

    /// create blocking flash driver<br />
    /// more see [Flash::new_blocking]
    fn build_blocking<const FLASH_SIZE: usize>(self) -> Flash<'d, FLASH, Blocking, FLASH_SIZE>;
}

/// flush support Flash trait
impl<'d> FlashTrait<'d> for Peri<'d, FLASH> {
    #[inline]
    fn build<const FLASH_SIZE: usize>(self, dma: Peri<'d, impl Channel>) -> Flash<'d, FLASH, Async, FLASH_SIZE> {
        Flash::new(self, dma)
    }

    #[inline]
    fn build_blocking<const FLASH_SIZE: usize>(self) -> Flash<'d, FLASH, Blocking, FLASH_SIZE> {
        Flash::new_blocking(self)
    }
}
