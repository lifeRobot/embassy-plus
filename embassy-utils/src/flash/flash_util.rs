use embassy_rp_plus::embassy_rp::flash;
use embassy_rp_plus::embassy_rp::flash::{Instance, Mode};
use crate::flash::err::FlashResult;
use crate::flash::flash_lock::FlashLock;

/// flash util
pub struct FlashUtil<'a, T: Instance, M: Mode, const FLASH_SIZE: usize> {
    /// flash device
    pub flash: &'a FlashLock<'a, T, M, FLASH_SIZE>,
    /// offset from the flash start, NOT an absolute address.
    pub offset: u32,
    /// flash minimum erase size
    erase_size: u32,
}

/// custom method
impl<'a, T: Instance, M: Mode, const FLASH_SIZE: usize> FlashUtil<'a, T, M, FLASH_SIZE> {
    /// create flash util
    #[inline]
    pub fn new(flash: &'a FlashLock<'a, T, M, FLASH_SIZE>, offset: u32, erase_size: u32) -> Self {
        Self { flash, offset, erase_size }
    }

    /// create flash util, default offset is 0x100000, default erase_size is 4096
    #[inline]
    pub fn new_default(flash: &'a FlashLock<'a, T, M, FLASH_SIZE>) -> Self {
        Self::new(flash, 0x100000, 4096)
    }

    /// try read to buf, more see [FlashLock::blocking_read]
    #[inline]
    pub async fn try_read(&self, buf: &mut [u8]) -> Result<(), flash::Error> {
        self.flash.blocking_read(self.offset, buf).await
    }

    /// try erase, more see [FlashLock::blocking_erase]
    pub async fn try_erase(&self, num: u32) -> Result<(), flash::Error> {
        if num == 0 { return Ok(()); }
        self.flash.blocking_erase(self.offset, self.offset + self.erase_size * num).await
    }

    /// try erase and write data<br />
    /// flash memories with lengths multiple of erase_size will be automatically erased.
    /// ff data is not empty, at least one flash memory of erase_size block will be erased
    pub async fn try_erase_write(&self, buf: &[u8]) -> FlashResult<()> {
        if buf.is_empty() { return Ok(()); }

        // calc to offset address
        let len = u32::try_from(buf.len())?;
        let sub_to = if len % self.offset == 0 { 0 } else { 1 };
        let to = self.erase_size * (len / self.offset + sub_to) + self.offset;

        // erase and write
        self.flash.try_erase_write(self.offset, to, buf).await?;
        Ok(())
    }

    /// erase and write data, more see [Self::try_erase_write]<br />
    /// flash memories with lengths multiple of erase_size will be automatically erased.
    /// ff data is not empty, at least one flash memory of erase_size block will be erased
    #[inline]
    pub async fn erase_write(&self, buf: &[u8]) {
        self.try_erase_write(buf).await.ok();
    }
}
