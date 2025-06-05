use embassy_rp_plus::embassy_rp::flash::{Flash, Instance, Mode};

/// flash util
pub struct FlashUtil<'a, T: Instance, M: Mode, const FLASH_SIZE: usize> {
    /// flash device
    pub flash: &'a mut Flash<'a, T, M, FLASH_SIZE>,
    /// offset from the flash start, NOT an absolute address.
    pub offset: u32,
    /// flash minimum erase size
    erase_size: u32,
}

/// support call flash function
impl<'a, T: Instance, M: Mode, const FLASH_SIZE: usize> AsRef<Flash<'a, T, M, FLASH_SIZE>> for FlashUtil<'a, T, M, FLASH_SIZE> {
    #[inline]
    fn as_ref(&self) -> &Flash<'a, T, M, FLASH_SIZE> {
        self.flash
    }
}

/// support call flash mut function
impl<'a, T: Instance, M: Mode, const FLASH_SIZE: usize> AsMut<Flash<'a, T, M, FLASH_SIZE>> for FlashUtil<'a, T, M, FLASH_SIZE> {
    #[inline]
    fn as_mut(&mut self) -> &mut Flash<'a, T, M, FLASH_SIZE> {
        self.flash
    }
}

/// custom method
impl<'a, T: Instance, M: Mode, const FLASH_SIZE: usize> FlashUtil<'a, T, M, FLASH_SIZE> {
    /// create flash util
    #[inline]
    pub fn new(flash: &'a mut Flash<'a, T, M, FLASH_SIZE>, offset: u32, erase_size: u32) -> Self {
        Self { flash, offset, erase_size }
    }

    /// create flash util, default offset is 0x100000, default erase_size is 4096
    #[inline]
    pub fn new_default(flash: &'a mut Flash<'a, T, M, FLASH_SIZE>) -> Self {
        Self::new(flash, 0x100000, 4096)
    }
}
