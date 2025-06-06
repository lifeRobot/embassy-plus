use embassy_rp_plus::embassy_rp::flash::{Flash, Instance, Mode};
use crate::flash::flash_lock::FlashLock;

/// to flash lock
pub trait ToFlashLock<'a, T: Instance, M: Mode, const FLASH_SIZE: usize> {
    /// to flash lock
    fn to_lock(self) -> FlashLock<'a, T, M, FLASH_SIZE>;
}

/// flash support to lock
impl<'a, T: Instance, M: Mode, const FLASH_SIZE: usize> ToFlashLock<'a, T, M, FLASH_SIZE> for Flash<'a, T, M, FLASH_SIZE> {
    #[inline]
    fn to_lock(self) -> FlashLock<'a, T, M, FLASH_SIZE> {
        FlashLock::new(self)
    }
}
