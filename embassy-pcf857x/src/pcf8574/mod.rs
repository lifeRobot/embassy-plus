use embassy_rp::gpio::Level;
use embassy_rp::i2c::{Async, Instance, Mode};
use crate::i2c_lock::I2cLock;
use crate::pcf8574::pcf8574_pin::Pcf8574Pin;

pub mod pcf8574_pin;

/// pcf8575
pub struct PCF8574<'a, T: Instance, M: Mode> {
    /// i2c lock
    i2c: &'a I2cLock<'a, T, M>,
    /// i2c address
    address: u8,
    /// write buf
    write_buf: [u8; 1],
    /// read buf
    pub read_buf: [u8; 1],
}

/// custom method
impl<'a, T: Instance, M: Mode> PCF8574<'a, T, M> {
    /// create pcf8574
    #[inline]
    pub const fn new(i2c: &'a I2cLock<'a, T, M>, address: u8) -> Self {
        Self { i2c, address, write_buf: [0], read_buf: [0] }
    }

    /// create default address pcf8574, default address is 0x20
    #[inline]
    pub const fn new_default(i2c: &'a I2cLock<'a, T, M>) -> Self {
        Self::new(i2c, 0x20)
    }

    /// read to buf, see `self.read_buf`
    #[inline]
    pub async fn read_to_buf(&mut self) {
        self.i2c.blocking_read(self.address, &mut self.read_buf).await.ok();
    }

    /// read all gpio
    #[inline]
    pub async fn read_gpio(&mut self) -> u8 {
        self.read_to_buf().await;
        self.read_buf[0]
    }

    /// read pin level
    #[inline]
    pub async fn read_pin(&mut self, pin: Pcf8574Pin) -> Level {
        self.read_to_buf().await;
        ((self.read_buf[0] >> pin.to_u8() & 0x1) == 1).into()
    }

    /// write all buf<br />
    pub async fn write_buf(&mut self, buf: u8) {
        self.write_buf[0] = buf;
        self.re_write().await;
    }

    /// re-write all buf to i2c
    #[inline]
    pub async fn re_write(&self) {
        self.i2c.blocking_write(self.address, &self.write_buf).await.ok();
    }

    /// just set up the cache, no data will be written to i2c<br />
    /// if you need to write cached data to i2c, call[Self::re_write] or [Self::re_write_async]
    pub fn set_pin(&mut self, pin: Pcf8574Pin, level: Level) {
        match level {
            Level::High => { self.write_buf[0] |= 1 << pin.to_u8(); }
            Level::Low => { self.write_buf[0] &= !(1 << pin.to_u8()); }
        }
    }

    /// write one pin io
    #[inline]
    pub async fn write_pin(&mut self, pin: Pcf8574Pin, val: Level) {
        self.set_pin(pin, val);
        self.re_write().await;
    }

    /// write all pin to low
    pub async fn write_all_low(&mut self) {
        self.write_buf[0] = 0x00;
        self.i2c.blocking_write(self.address, &self.write_buf).await.ok();
    }

    /// write all pin to high
    pub async fn write_all_high(&mut self) {
        self.write_buf[0] = 0xFF;
        self.i2c.blocking_write(self.address, &self.write_buf).await.ok();
    }

    /// write pins level
    pub async fn write_pins(&mut self, pins: &[(Pcf8574Pin, Level)]) {
        for (pin, level) in pins {
            self.set_pin(*pin, *level);
        }

        self.re_write().await;
    }

    /// write pins to low
    pub async fn write_pins_low(&mut self, pins: &[Pcf8574Pin]) {
        for pin in pins {
            self.write_buf[0] &= !(1 << pin.to_u8());
        }

        self.re_write().await;
    }

    /// write pins to high
    pub async fn write_pins_high(&mut self, pins: &[Pcf8574Pin]) {
        for pin in pins {
            self.write_buf[0] |= 1 << pin.to_u8();
        }

        self.re_write().await;
    }
}

/// custom method
impl<'a, T: Instance> PCF8574<'a, T, Async> {
    /// read to buf, see `self.read_buf`
    #[inline]
    pub async fn read_to_buf_async(&mut self) {
        self.i2c.read_async(self.address, &mut self.read_buf).await.ok();
    }

    /// read all gpio
    #[inline]
    pub async fn read_gpio_async(&mut self) -> u8 {
        self.read_to_buf_async().await;
        self.read_buf[0]
    }

    /// read pin level
    #[inline]
    pub async fn read_pin_async(&mut self, pin: Pcf8574Pin) -> Level {
        self.read_to_buf_async().await;
        ((self.read_buf[0] >> pin.to_u8() & 0x1) == 1).into()
    }


    /// write all buf<br />
    pub async fn write_buf_async(&mut self, buf: u8) {
        self.write_buf[0] = buf;
        self.re_write_async().await;
    }

    /// re-write all buf to i2c
    #[inline]
    pub async fn re_write_async(&self) {
        self.i2c.write_async(self.address, self.write_buf).await.ok();
    }
}
