use embassy_rp::i2c::{Async, Error, I2c, Instance, Mode};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use crate::pcf8575::PCF8575;

pub mod to_lock;

/// i2c with lock
pub struct I2cLock<'a, T: Instance, M: Mode> {
    /// i2c device
    i2c: Mutex<CriticalSectionRawMutex, I2c<'a, T, M>>,
}

/// custom method
impl<'a, T: Instance, M: Mode> I2cLock<'a, T, M> {
    /// create i2c lock
    pub const fn new(i2c: I2c<'a, T, M>) -> Self {
        Self { i2c: Mutex::new(i2c) }
    }

    /// build pcf8575
    #[inline]
    pub const fn build_pcf8575_default(&'a self) -> PCF8575<'a, T, M> {
        PCF8575::new_default(self)
    }

    /// build pcf8575
    #[inline]
    pub const fn build_pcf8575(&'a self, address: u8) -> PCF8575<'a, T, M> {
        PCF8575::new(self, address)
    }

    /// Read from address into buffer blocking caller until done.<br />
    /// more see [I2c::blocking_read]
    #[inline]
    pub async fn blocking_read(&self, address: impl Into<u16>, read: &mut [u8]) -> Result<(), Error> {
        self.i2c.lock().await.blocking_read(address, read)
    }

    /// Write to address from buffer blocking caller until done.<br />
    /// more see [I2c::blocking_write]
    #[inline]
    pub async fn blocking_write(&self, address: impl Into<u16>, write: &[u8]) -> Result<(), Error> {
        self.i2c.lock().await.blocking_write(address, write)
    }
}

/// custom method
impl<'a, T: Instance> I2cLock<'a, T, Async> {
    /// Write to address from buffer asynchronously.
    #[inline]
    pub async fn write_async(&self, address: impl Into<u16>, bytes: impl IntoIterator<Item=u8>) -> Result<(), Error> {
        self.i2c.lock().await.write_async(address, bytes).await
    }

    /// Read from address into buffer asynchronously.
    #[inline]
    pub async fn read_async(&self, address: impl Into<u16>, bytes: &mut [u8]) -> Result<(), Error> {
        self.i2c.lock().await.read_async(address, bytes).await
    }

    /// Write to address from bytes and read from address into buffer asynchronously.
    #[inline]
    pub async fn write_read_async(
        &self,
        addr: impl Into<u16>,
        bytes: impl IntoIterator<Item=u8>,
        buffer: &mut [u8]) -> Result<(), Error> {
        self.i2c.lock().await.write_read_async(addr, bytes, buffer).await
    }
}
