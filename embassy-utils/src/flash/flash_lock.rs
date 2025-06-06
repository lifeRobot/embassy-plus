use embassy_rp_plus::embassy_rp::flash;
use embassy_rp_plus::embassy_rp::flash::{Async, Flash, Instance, Mode};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use crate::flash::flash_util::FlashUtil;

/// flash lock
pub struct FlashLock<'a, T: Instance, M: Mode, const FLASH_SIZE: usize> {
    /// flash
    flash: Mutex<CriticalSectionRawMutex, Flash<'a, T, M, FLASH_SIZE>>,
}

/// custom method
impl<'a, T: Instance, M: Mode, const FLASH_SIZE: usize> FlashLock<'a, T, M, FLASH_SIZE> {
    /// create flash lock
    #[inline]
    pub fn new(flash: Flash<'a, T, M, FLASH_SIZE>) -> Self {
        Self { flash: Mutex::new(flash) }
    }

    /// build flash util, more see [FlashUtil::new]
    #[inline]
    pub fn build_flash_util(&'a self, offset: u32, erase_size: u32) -> FlashUtil<'a, T, M, FLASH_SIZE> {
        FlashUtil::new(self, offset, erase_size)
    }

    /// build flash util default, more see [FlashUtil::new_default]
    #[inline]
    pub fn build_flash_util_default(&'a self) -> FlashUtil<'a, T, M, FLASH_SIZE> {
        FlashUtil::new_default(self)
    }

    /// Flash capacity. more see [Flash::capacity]
    #[inline]
    pub async fn capacity(&self) -> usize {
        self.flash.lock().await.capacity()
    }

    /// Read SPI flash JEDEC ID, more see [Flash::blocking_jedec_id]
    #[inline]
    pub async fn blocking_jedec_id(&self) -> Result<u32, flash::Error> {
        self.flash.lock().await.blocking_jedec_id()
    }

    /// Read SPI flash unique ID, more see [Flash::blocking_unique_id]
    #[inline]
    pub async fn blocking_unique_id(&self, uid: &mut [u8]) -> Result<(), flash::Error> {
        self.flash.lock().await.blocking_unique_id(uid)
    }

    /// Blocking read. more see [Flash::blocking_read]
    #[inline]
    pub async fn blocking_read(&self, offset: u32, bytes: &mut [u8]) -> Result<(), flash::Error> {
        self.flash.lock().await.blocking_read(offset, bytes)
    }

    /// Blocking erase. more see [Flash::blocking_erase]
    #[inline]
    pub async fn blocking_erase(&self, from: u32, to: u32) -> Result<(), flash::Error> {
        self.flash.lock().await.blocking_erase(from, to)
    }

    /// Blocking write. more see [Flash::blocking_write]
    #[inline]
    pub async fn blocking_write(&self, offset: u32, bytes: &[u8]) -> Result<(), flash::Error> {
        self.flash.lock().await.blocking_write(offset, bytes)
    }

    /// try erase and write, flash memory in areas [offset - to] will be erased<br />
    /// note that the area length of [offset - to] needs to meet the multiple relationship of the erase block size<br />
    /// such as rp2040 (to - offset) % 4096 must equal 0, because the minimum erase block size of rp2040 flash memory is 4096
    pub async fn try_erase_write(&self, offset: u32, to: u32, buf: &[u8]) -> Result<(), flash::Error> {
        let mut flash = self.flash.lock().await;
        flash.blocking_erase(offset, to)?;
        flash.blocking_write(offset, buf)
    }
}

/// custom method
impl<'a, T: Instance, const FLASH_SIZE: usize> FlashLock<'a, T, Async, FLASH_SIZE> {
    /// Async read. more see [Flash::<'a,T,Async,FLASH_SIZE>::read]
    #[inline]
    pub async fn read(&self, offset: u32, bytes: &mut [u8]) -> Result<(), flash::Error> {
        self.flash.lock().await.read(offset, bytes).await
    }
}
