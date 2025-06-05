use embassy_rp::dma::Channel;
use embassy_rp::flash::{Async, Blocking, Flash};
use embassy_rp::Peripheral;
use embassy_rp::peripherals::FLASH;

/// flash trait
pub trait FlashTrait<'d>: Peripheral<P=FLASH> + 'd {
    /// create flash driver<br />
    /// more see [Flash::new]
    #[inline]
    fn build<const FLASH_SIZE: usize>(self, dma: impl Peripheral<P=impl Channel> + 'd) -> Flash<'d, FLASH, Async, FLASH_SIZE> {
        Flash::new(self, dma)
    }

    /// create blocking flash driver<br />
    /// more see [Flash::new_blocking]
    #[inline]
    fn build_blocking<const FLASH_SIZE: usize>(self) -> Flash<'d, FLASH, Blocking, FLASH_SIZE> {
        Flash::new_blocking(self)
    }
}

/// flush support Flash trait
impl FlashTrait<'_> for FLASH {}
